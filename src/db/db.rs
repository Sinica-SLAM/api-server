use chrono::Utc;
use entity::{
    results,
    users::{self, Entity as User},
};
use sea_orm::{prelude::*, DbErr, NotSet, Set};

pub async fn get_first_user(conn: &DatabaseConnection) -> Result<Option<users::Model>, DbErr> {
    User::find().one(conn).await
}

pub async fn create_user(
    conn: &DatabaseConnection,
    username: &str,
    max_query: i32,
) -> Result<users::Model, DbErr> {
    users::ActiveModel {
        id: NotSet,
        username: Set(username.to_string()),
        remain: Set(max_query),
        max_query: Set(max_query),
        reset_at: NotSet,
    }
    .insert(conn)
    .await
}

pub async fn find_user_by_id(
    conn: &DatabaseConnection,
    id: i32,
) -> Result<Option<users::Model>, DbErr> {
    User::find_by_id(id).one(conn).await
}

pub async fn set_user_remain(
    conn: &DatabaseConnection,
    value: i32,
    user: users::Model,
) -> Result<users::Model, DbErr> {
    let mut user: users::ActiveModel = user.into();
    user.remain = Set(value);
    user.reset_at = Set(Utc::now());

    user.update(conn).await
}

pub async fn create_result(
    conn: &DatabaseConnection,
    id: String,
    owner_id: i32,
) -> Result<results::Model, DbErr> {
    results::ActiveModel {
        id: Set(id),
        owner_id: Set(owner_id),
        file_path: NotSet,
        status: Set(results::Status::Running),
    }
    .insert(conn)
    .await
}

pub async fn find_result_by_id(
    conn: &DatabaseConnection,
    id: String,
) -> Result<Option<results::Model>, DbErr> {
    results::Entity::find_by_id(id).one(conn).await
}

pub async fn set_result_status(
    conn: &DatabaseConnection,
    value: results::Status,
    result: results::Model,
) -> Result<results::Model, DbErr> {
    let mut result: results::ActiveModel = result.into();
    result.status = Set(value);

    result.update(conn).await
}

pub async fn set_result_file_path(
    conn: &DatabaseConnection,
    value: String,
    result: results::Model,
) -> Result<results::Model, DbErr> {
    let mut result: results::ActiveModel = result.into();
    result.file_path = Set(Some(value));

    result.update(conn).await
}
