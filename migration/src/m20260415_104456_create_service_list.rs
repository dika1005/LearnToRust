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
                    .table(ServiceList::Table)
                    .if_not_exists()
                    .col(pk_auto(ServiceList::Id))
                    .col(string(ServiceList::ServiceName).not_null())
                    .col(big_integer(ServiceList::ServiceFee).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(ServiceList::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ServiceList {
    Table,
    Id,
    ServiceName,
    ServiceFee,
}
