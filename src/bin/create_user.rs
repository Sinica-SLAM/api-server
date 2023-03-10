use api_server::db::create_user;
use clap::Parser;
use entity::sea_orm::{Database, DatabaseConnection};
#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long)]
    pub username: String,

    ///max query per day
    #[clap(short, long)]
    pub max_query: i32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let db: DatabaseConnection = Database::connect("sqlite://db.sqlite3")
        .await
        .expect("db connect fail");

    let _ = create_user(&db, args.username.as_str(), args.max_query).await;
    println!("\nSaved user {}", args.username);
}
