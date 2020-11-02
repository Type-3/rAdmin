pub mod cli;
pub mod factories;
pub mod forms;
pub mod guards;
pub mod models;
pub mod requests;
pub mod routes;
pub mod schema;
pub mod traits;

mod db;
pub use self::db::AclDbMod;

mod auth;
pub use self::auth::Auth;

mod seeder;
pub use self::seeder::{run_init_seeders, ServerSeeder};

pub struct AclModule;

impl crate::modules::ServerModule for AclModule {
    fn identifier(&self) -> &'static str {
        "AclModule"
    }

    fn version(&self) -> &'static str {
        clap::crate_version!()
    }

    fn cli(&self) -> Box<dyn crate::modules::CliModule> {
        Box::new(cli::AclCli)
    }

    fn database(&self) -> Box<dyn crate::modules::DatabaseModule> {
        Box::new(AclDbMod)
    }

    fn routes(&self) -> Box<dyn crate::modules::RoutesModule> {
        Box::new(routes::AclRoutesMod)
    }
}
