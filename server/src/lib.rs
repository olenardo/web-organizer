mod routes;
mod controllers;
mod utils;

use std::net::TcpListener;

use crate::utils::clap::Opt;

use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;

pub async fn run() {
    
    let opt = Opt::parse();

    // logs init
    if opt.log {
        tracing_subscriber::fmt::init();
        tracing::info!("LOGS: ENABLED");
    }

    // Service
    let app = routes::routes()
        .await
        .into_make_service();

    // Listener
    let tcp_listener = TcpListener::bind(
        (opt.ip, opt.port)
    )
        .unwrap();

    // Runner
    if let (Some(cert), Some(key)) = (opt.cert, opt.key) {

        // TLS Config
        let tls_cfg = RustlsConfig::from_pem_file(
            cert, key
        )
            .await
            .unwrap();

        tracing::info!("PROTOCOL: HTTPS");

        // Serve
        axum_server::from_tcp_rustls(
            tcp_listener,
            tls_cfg
        )
            .serve(app)
            .await
            .unwrap();
    
    // IF NO SERTIFICATE WAS PROVIDED
    } else {

        tracing::info!("PROTOCOL: HTTP");

        axum_server::from_tcp(tcp_listener)
            .serve(app)
            .await
            .unwrap();
    }
}