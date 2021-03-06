mod delete;
mod update;
mod select;
mod store;
mod table;

use crate::select::ApiSelect;
use crate::table::ApiTable;
use crate::crud::CrudStore;

pub fn api_routes() -> Vec<rocket::Route> {
    let mut routes = rocket::routes![update::update, delete::api_delete];
    routes.push(store::RolesStore::route());
    routes.push(select::RoleSelect::route());
    routes.push(table::RolesTable::route());
    routes
}

pub fn form_routes() -> Vec<rocket::Route> {
    rocket::routes![delete::form_delete]
}
