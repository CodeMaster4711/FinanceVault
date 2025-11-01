use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subscription::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Subscription::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Subscription::UserId).uuid().not_null())
                    .col(ColumnDef::new(Subscription::Name).string().not_null())
                    .col(ColumnDef::new(Subscription::Amount).decimal().not_null())
                    .col(
                        ColumnDef::new(Subscription::BillingCycle)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Subscription::NextBillingDate)
                            .date_time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Subscription::Category).string().not_null())
                    .col(
                        ColumnDef::new(Subscription::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_subscription_user")
                            .from(Subscription::Table, Subscription::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subscription::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Subscription {
    Table,
    Id,
    UserId,
    Name,
    Amount,
    BillingCycle,
    NextBillingDate,
    Category,
    IsActive,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
