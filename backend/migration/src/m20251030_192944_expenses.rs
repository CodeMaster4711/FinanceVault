use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Expenses::Table)
                    .if_not_exists()
                    .col(pk_uuid(Expenses::Id))
                    .col(string(Expenses::Description))
                    .col(decimal(Expenses::Amount))
                    .col(date_time(Expenses::Date))
                    .col(string(Expenses::Category))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Expenses::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Expenses {
    Table,
    Id,
    Description,
    Amount,
    Date,
    Category,
}
