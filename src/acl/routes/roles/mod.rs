mod delete;
mod update;
mod select;
mod store;
mod table;

use crate::select::ApiSelect;
use crate::table::ApiTable;
use crate::crud::{CrudDelete, CrudStore};

pub fn api_routes() -> Vec<rocket::Route> {
    let mut routes = rocket::routes![update::update];
    routes.push(delete::RolesDelete::route());
    routes.push(store::RolesStore::route());
    routes.push(select::RoleSelect::route());
    routes.push(table::RolesTable::route());
    routes
}
