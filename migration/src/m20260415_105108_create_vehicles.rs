use sea_orm_migration::{prelude::*, schema::*};
use super::m20260415_105052_create_customers::Customers;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                   .table(Vehicles::Table)
                .if_not_exists()
                .col(pk_auto(Vehicles::Id))
                .col(integer(Vehicles::CustomerId).not_null()) // Foreign Key
                .col(string(Vehicles::PlateNumber).unique_key().not_null())
                .col(string(Vehicles::Brand).not_null())
                .col(string(Vehicles::Model).not_null())
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-vehicle-customer")
                        .from(Vehicles::Table, Vehicles::CustomerId)
                        .to(Customers::Table, Customers::Id)
                        .on_delete(ForeignKeyAction::Cascade) 
                )
                .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Vehicles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Vehicles {
    Table,
    Id,
    CustomerId,
    PlateNumber,
    Brand,
    Model,
}
