pub use sea_orm_migration::prelude::*;

mod m20241016_075756_users;
mod m20241016_075850_permissions;
mod m20241016_075850_settings;
mod m20241016_101754_houses;
mod m20241021_143934_credentials;
mod m20241021_143946_expenses;
mod m20241128_102905_providers;
mod m20241128_132112_home_insurance;
mod m20241128_132448_pets;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241016_075756_users::Migration),
            Box::new(m20241016_075850_permissions::Migration),
            Box::new(m20241016_075850_settings::Migration),
            Box::new(m20241016_101754_houses::Migration),
            Box::new(m20241021_143934_credentials::Migration),
            Box::new(m20241021_143946_expenses::Migration),
            Box::new(m20241128_102905_providers::Migration),
            Box::new(m20241128_132112_home_insurance::Migration),
            Box::new(m20241128_132448_pets::Migration),
        ]
    }
}
