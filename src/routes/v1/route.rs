use axum::{extract::DefaultBodyLimit, routing::post, Router};

use super::handler;

pub fn create_route() -> Router {
    Router::new()
        .route("/rec_upload", post(handler::rec_upload))
        .route("/align_upload", post(handler::align_upload))
        .route("/app_upload", post(handler::app_upload))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        .route("/rec_youtube", post(handler::rec_youtube))
}
