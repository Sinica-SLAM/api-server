use crate::routes::{v1, v2};
use axum::{Extension, Router};
use entity::sea_orm::DatabaseConnection;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{ CorsLayer, Any},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

pub fn create_route(conn: &DatabaseConnection, auth: &bool) -> Router {
    Router::new()
        .nest("/api", v1::create_route())
        .nest("/api/v2", v2::create_route(auth))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_request(DefaultOnRequest::new().level(Level::INFO))
                        .on_response(DefaultOnResponse::new().level(Level::INFO)),
                )
                .layer(CompressionLayer::new())
                .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
                .layer(Extension(conn.clone())),
        )
}
