pub use sea_orm_migration::prelude::*;
extern crate dotenv;

mod m20220903_102025_members;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220903_102025_members::Migration)]
    }
}
