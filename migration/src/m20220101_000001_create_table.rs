use sea_orm_migration::prelude::*;

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
                    // Kolom ID: Integer, Primary Key, Auto Increment
                    .col(ColumnDef::new(Spareparts::Id).integer().not_null().auto_increment().primary_key())
                    // Kolom Nama: String/Varchar, Tidak boleh kosong
                    .col(ColumnDef::new(Spareparts::Name).string().not_null())
                    // Kolom Stok: Integer, Tidak boleh kosong
                    .col(ColumnDef::new(Spareparts::Stock).integer().not_null())
                    // Kolom Harga: Decimal, Tidak boleh kosong
                    .col(ColumnDef::new(Spareparts::Price).decimal().not_null())
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
enum Spareparts {
    Table,
    Id,
    Name,
    Stock,
    Price,
}