use sea_orm_migration::{prelude::*, schema::*};
use super::m20260415_105108_create_vehicles::Vehicles;
use super::m20260414_163245_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                 .table(ServiceOrders::Table)
                .if_not_exists()
                .col(pk_auto(ServiceOrders::Id))
                .col(integer(ServiceOrders::VehicleId).not_null())
                .col(integer(ServiceOrders::MechanicId).not_null())
                .col(integer(ServiceOrders::FrontDeskId).not_null())
                .col(string(ServiceOrders::Status).not_null().default("queued"))
                .col(big_integer(ServiceOrders::TotalPrice).not_null().default(0))
                .col(date_time(ServiceOrders::CreatedAt).not_null().default(Expr::current_timestamp()))
                // Foreign Key ke Vehicles
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-order-vehicle")
                        .from(ServiceOrders::Table, ServiceOrders::VehicleId)
                        .to(Vehicles::Table, Vehicles::Id)
                )
                // Foreign Key ke Users (Mechanic)
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-order-mechanic")
                        .from(ServiceOrders::Table, ServiceOrders::MechanicId)
                        .to(Users::Table, Users::Id)
                )
                .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(ServiceOrders::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ServiceOrders {
    Table,
    Id,
    VehicleId,
    MechanicId,
    FrontDeskId,
    Status,
    TotalPrice,
    CreatedAt,
}
