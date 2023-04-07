use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Results::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Results::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Results::OwnerId).integer().not_null())
                    .col(ColumnDef::new(Results::FilePath).string())
                    .col(
                        ColumnDef::new(Results::Status)
                            .enumeration(
                                Status::Table,
                                [Status::Running, Status::Fail, Status::Complete],
                            )
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_result_owner")
                            .from(Results::Table, Results::OwnerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Results::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Results {
    Table,
    Id,
    OwnerId,
    FilePath,
    Status,
}

#[derive(Iden, EnumIter)]
enum Status {
    Table,
    #[iden = "RUNNING"]
    Running,
    #[iden = "FAIL"]
    Fail,
    #[iden = "COMPLETE"]
    Complete,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
