use sea_orm_migration::{prelude::*, schema::*};
use super::m20260415_105313_create_service_orders::ServiceOrders;
use super::m20220101_000001_create_table::Spareparts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(ServiceOrderItems::Table)
                .if_not_exists()
                .col(pk_auto(ServiceOrderItems::Id))
                .col(integer(ServiceOrderItems::ServiceOrderId).not_null())
                .col(integer(ServiceOrderItems::SparepartId).not_null())
                .col(integer(ServiceOrderItems::Quantity).not_null())
                .col(big_integer(ServiceOrderItems::PriceAtTime).not_null()) // Harga saat transaksi
                // Foreign Key ke ServiceOrders
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-item-order")
                        .from(ServiceOrderItems::Table, ServiceOrderItems::ServiceOrderId)
                        .to(ServiceOrders::Table, ServiceOrders::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                )
                // Foreign Key ke Spareparts
                .foreign_key(
                    ForeignKey::create()
                        .name("fk-item-sparepart")
                        .from(ServiceOrderItems::Table, ServiceOrderItems::SparepartId)
                        .to(Spareparts::Table, Spareparts::Id)
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(ServiceOrderItems::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ServiceOrderItems {
    Table,
    Id,
    ServiceOrderId,
    SparepartId,
    Quantity,
    PriceAtTime,
}
