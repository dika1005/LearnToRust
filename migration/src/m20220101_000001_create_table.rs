use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Fungsi 'up' dijalankan saat kita membuat tabel
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                .table(Spareparts::Table)
                .if_not_exists()
                .col(pk_auto(Spareparts::Id))
                .col(string(Spareparts::SkuCode).unique_key().not_null())
                .col(string(Spareparts::Name).not_null())
                .col(integer(Spareparts::Stock).not_null().default(0))
                .col(big_integer(Spareparts::Price).not_null())
                .to_owned(),
            )
            .await
    }

    // Fungsi 'down' dijalankan jika kita ingin membatalkan (rollback) tabel ini
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Spareparts::Table).to_owned())
            .await
    }
}

// Definisikan nama tabel dan kolom di sini agar tidak typo
#[derive(DeriveIden)]
pub enum Spareparts {
    Table,
    Id,
    SkuCode,
    Name,
    Stock,
    Price,
}