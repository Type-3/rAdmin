mod login;
pub mod logout;
pub mod register;

pub fn api_routes() -> Vec<rocket::Route> {
    rocket::routes![login::login, logout::logout, register::register]
}
