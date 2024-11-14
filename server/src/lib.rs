use axum::{routing::get, Router};
use tokio::net::TcpListener;

pub async fn run() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(|| async move {
            String::from("Response from the server")
        }))
            .into_make_service();
    
    axum::serve(
        listener,
        app
    )
        .await
        .unwrap();
}