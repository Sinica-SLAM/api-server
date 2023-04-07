use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;
use tower_http::auth::AsyncRequireAuthorizationLayer;

use super::authorization::TokenAuth;
use super::handler;

pub fn create_route(auth: &bool, conn: &DatabaseConnection) -> Router {
    Router::new()
        .route("/rec_upload", post(handler::rec_upload))
        .route("/align_upload", post(handler::align_upload))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        // .route("/app_upload", post(handler::app_upload))
        .route("/rec_youtube", post(handler::rec_youtube))
        .route("/translation", post(handler::translation))
        .layer(AsyncRequireAuthorizationLayer::new(TokenAuth {
            auth: *auth,
            conn: conn.clone(),
        }))
        .route("/result/:id", get(handler::result))
        .with_state(conn.clone())
}
