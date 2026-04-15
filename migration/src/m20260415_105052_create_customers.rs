use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Customers::Table)
                    .if_not_exists()
                    .col(pk_auto(Customers::Id))
                    .col(string(Customers::Name).not_null())
                    .col(string(Customers::PhoneNumber).not_null())
                    .col(string(Customers::Address))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Customers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Customers {
    Table,
    Id,
    Name,
    PhoneNumber,
    Address,
}
