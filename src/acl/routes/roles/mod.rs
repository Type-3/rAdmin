mod create;
mod delete;
mod edit;
mod select;
mod table;

pub fn api_routes() -> Vec<rocket::Route> {
    rocket::routes![
        table::data,
        create::store,
        edit::update,
        delete::delete,
        select::options
    ]
}
