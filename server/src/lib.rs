mod routes;

use tokio::net::TcpListener;

pub async fn run() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    let app = routes::routes()
        .await
        .into_make_service();
    
    axum::serve(
        listener,
        app
    )
        .await
        .unwrap();
}