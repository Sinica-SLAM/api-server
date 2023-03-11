use super::authorization::TokenAuth;
use super::handler;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::auth::AsyncRequireAuthorizationLayer;
use entity::sea_orm::DatabaseConnection;


pub fn create_route(auth: &bool, conn: &DatabaseConnection) -> Router {
     Router::new()
        .route("/rec_upload", post(handler::rec_upload))
        .route("/align_upload", post(handler::align_upload))
        // .route("/app_upload", post(handler::app_upload))
        .route("/rec_youtube", post(handler::rec_youtube))
        .route("/translation", post(handler::translation))
        .layer(AsyncRequireAuthorizationLayer::new(TokenAuth{auth: *auth, conn}))
        .route("/result/:id", get(handler::result))
        .with_state(conn.clone())
}