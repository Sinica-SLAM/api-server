use std::net::SocketAddr;

use api_server::{config::Config, routes::create_route};
use clap::Parser;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

#[tokio::main]
async fn main() {
    let config = Config::parse();
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("{addr}");
    let conn: DatabaseConnection = Database::connect(&config.database_url)
        .await
        .expect("db connect fail");
    Migrator::up(&conn, None).await.expect("migration fail");

    axum::Server::bind(&addr)
        .serve(create_route(&conn, &config.auth).into_make_service())
        .await
        .unwrap();
}
