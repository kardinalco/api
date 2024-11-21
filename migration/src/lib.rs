pub use sea_orm_migration::prelude::*;

mod m20241016_075756_users;
mod m20241016_075850_permissions;
mod m20241016_101754_houses;
mod m20241021_143934_credentials;
mod m20241021_143946_expenses;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241016_075756_users::Migration),
            Box::new(m20241016_075850_permissions::Migration),
            Box::new(m20241016_101754_houses::Migration),
            Box::new(m20241021_143934_credentials::Migration),
            /*Box::new(m20241021_143946_expenses::Migration),*/
        ]
    }
}
