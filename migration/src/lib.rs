pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260414_163245_create_users;
mod m20260415_104456_create_service_list;
mod m20260415_105052_create_customers;
mod m20260415_105108_create_vehicles;
mod m20260415_105313_create_service_orders;
mod m20260415_105321_create_service_order_items;
mod m20260421_141951_add_token_to_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260414_163245_create_users::Migration),
            Box::new(m20260415_104456_create_service_list::Migration),
            Box::new(m20260415_105052_create_customers::Migration),
            Box::new(m20260415_105108_create_vehicles::Migration),
            Box::new(m20260415_105313_create_service_orders::Migration),
            Box::new(m20260415_105321_create_service_order_items::Migration),
            Box::new(m20260421_141951_add_token_to_users::Migration),
        ]
    }
}
