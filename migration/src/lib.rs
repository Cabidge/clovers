pub use sea_orm_migration::prelude::*;

mod m20230815_000001_create_post_table;
mod m20230819_163054_add_date_column;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230815_000001_create_post_table::Migration),
            Box::new(m20230819_163054_add_date_column::Migration),
        ]
    }
}
