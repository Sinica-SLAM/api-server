use entity::results::*;
use entity::users;

use sea_schema::migration::{sea_query::*, *};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220328_000001_create_results_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> std::result::Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(ColumnDef::new(Column::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Column::OwnerId).integer().not_null())
                    .col(ColumnDef::new(Column::FilePath).string())
                    .col(ColumnDef::new(Column::Status).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_result_owner")
                            .from(Entity, Column::OwnerId)
                            .to(users::Entity, users::Column::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> std::result::Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await
    }
}
