mod delete;
mod update;
mod store;
mod table;

use crate::table::ApiTable;
use crate::crud::{CrudDelete, CrudStore};

pub fn api_routes() -> Vec<rocket::Route> {
    let mut routes = rocket::routes![update::update];
    routes.push(delete::AccountsDelete::route());
    routes.push(store::AccountsStore::route());
    routes.push(table::AccountsTable::route());
    routes
}
