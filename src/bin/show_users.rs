use entity::{
    sea_orm::{Database, DatabaseConnection, EntityTrait},
    users::Entity as User,
};

#[tokio::main]
async fn main() {
    let db: DatabaseConnection = Database::connect("sqlite://db.sqlite3")
        .await
        .expect("db connect fail");
    let users = User::find().all(&db).await.expect("find all fail");
    for user in users {
        println!("{}", user.id);
        println!("----------\n");
        println!("{}", user.username);
    }
}
