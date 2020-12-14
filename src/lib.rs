#![feature(associated_type_defaults, decl_macro)]
#[macro_use]
pub extern crate diesel;
#[macro_use]
pub extern crate diesel_migrations;
pub extern crate chrono;
pub extern crate clap;
pub extern crate cli_table;
pub extern crate rocket;
pub extern crate rocket_contrib;
pub extern crate serde;
pub extern crate serde_json;
pub extern crate typed_builder;
pub extern crate uuid;
pub extern crate validator;
pub extern crate validator_derive;

mod application;
mod database;
mod error;
mod response;

pub mod acl;
pub mod select;
pub mod table;
pub mod cli;
pub mod client;
pub mod config;
pub mod crud;
pub mod modules;
pub mod roles;
pub mod traits;
pub mod types;

pub use self::application::Application;
pub use self::database::{establish_connection, DbConnection};
pub use self::error::ServerError;
pub use self::response::ApiResponse;
pub use clap::{crate_authors, crate_description, crate_name, crate_version};
pub use radmin_macros::{from_similar, Role};

use crate::modules::{CliModule, Modules};
use rocket::Rocket;

pub fn rocket_factory(conf: Option<&str>, modules: &Modules, app: Option<&Application>) -> Result<Rocket, ServerError> {
    let mut config = config::get_rocket_config(conf)?;

    for module in modules.0.iter() {
        config = (*module).config(config);
    }

    let mut server = rocket::custom(config.clone())
        .attach(DbConnection::fairing())
        .manage(config);

    let mut catchers = vec![];

    for module in modules.0.iter() {
        let routes = module.routes();
        for (path, route) in routes.routes() {
            server = server.mount(&format!("/{}", path), route);
        }
        catchers.extend(routes.catch());
    }
    server = server.register(catchers);

    if let Some(app) = app {
        let closure = app.configure.lock().unwrap();
        server = closure(server);
    }

    Ok(server)
}

pub fn run(app: Application) -> Result<(), ServerError> {
    let cli = cli::CliApp::new(&app);
    let mut args = cli.app().unwrap();
    for module in &app.modules.0 {
        if let Some(cli) = (*module).cli().app() {
            args = args.subcommand(cli);
        }
    }
    let matches = args.get_matches();
    cli.handle(Some(&matches))?;
    Ok(())
}
