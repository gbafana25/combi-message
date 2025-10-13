#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20251012_054437_messages;
mod m20251012_182002_add_device_name_tomessages;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20251012_054437_messages::Migration),
            Box::new(m20251012_182002_add_device_name_tomessages::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}