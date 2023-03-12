use super::authorization::TokenAuth;
use super::handler;
use axum::{
    routing::{get, post},
    Router, extract::DefaultBodyLimit,
};
use tower_http::auth::AsyncRequireAuthorizationLayer;
use sea_orm::DatabaseConnection;


pub fn create_route(auth: &bool, conn: &DatabaseConnection) -> Router {
     Router::new()
        .route("/rec_upload", post(handler::rec_upload))
        .route("/align_upload", post(handler::align_upload))
        .layer(DefaultBodyLimit::max(1024*1024*1024))
        // .route("/app_upload", post(handler::app_upload))
        .route("/rec_youtube", post(handler::rec_youtube))
        .route("/translation", post(handler::translation))
        .layer(AsyncRequireAuthorizationLayer::new(TokenAuth{auth: *auth, conn:conn.clone()}))
        .route("/result/:id", get(handler::result))
        .with_state(conn.clone())
}