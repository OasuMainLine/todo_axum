use axum::http;
use axum::routing::{get, Router};
pub fn get_router() -> Router {
    let router = Router::new().route("/", get(health));
    router
}

async fn health() -> http::StatusCode {
    http::StatusCode::OK
}
