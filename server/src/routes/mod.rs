use axum::{routing::get, Router};

pub async fn routes() -> Router {
    Router::new()
        .route("/", get(|| async move {
            String::from("Response from the server")
        }))
}