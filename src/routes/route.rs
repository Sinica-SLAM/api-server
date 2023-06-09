use axum::Router;
use sea_orm::DatabaseConnection;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::routes::{v1, v2};

pub fn create_route(conn: &DatabaseConnection, auth: &bool) -> Router {
    Router::new()
        .nest("/api", v1::create_route())
        .nest("/api/v2", v2::create_route(auth, conn))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_request(DefaultOnRequest::new().level(Level::INFO))
                        .on_response(DefaultOnResponse::new().level(Level::INFO)),
                )
                .layer(CompressionLayer::new())
                .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any)),
        )
}
