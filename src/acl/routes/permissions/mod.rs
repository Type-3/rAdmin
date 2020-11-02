mod create;
mod delete;
mod edit;
mod select;
mod table;

pub fn api_routes() -> Vec<rocket::Route> {
    rocket::routes![
        table::data,
        edit::update,
        create::store,
        delete::delete,
        select::options
    ]
}
