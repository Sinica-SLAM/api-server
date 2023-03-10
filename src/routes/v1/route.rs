use super::handler;
use axum::{routing::post, Router};

pub fn create_route() -> Router {
    Router::new()
        .route("/rec_upload", post(handler::rec_upload))
        .route("/align_upload", post(handler::align_upload))
        .route("/app_upload", post(handler::app_upload))
        .route("/rec_youtube", post(handler::rec_youtube))
}
