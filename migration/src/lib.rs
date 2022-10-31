pub use sea_orm_migration::prelude::*;
extern crate dotenv;

mod m20220903_102025_members;
mod m20220918_161145_committees;
mod m20220918_214426_independent_expenditures;
mod m20221005_181214_committee_contributions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220903_102025_members::Migration),
            Box::new(m20220918_161145_committees::Migration),
            Box::new(m20220918_214426_independent_expenditures::Migration),
            Box::new(m20221005_181214_committee_contributions::Migration),
        ]
    }
}
