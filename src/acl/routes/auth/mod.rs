use crate::acl::AclModuleConfig;

mod login;
pub mod logout;
pub mod register;

pub fn api_routes(config: &AclModuleConfig) -> Vec<rocket::Route> {
    if config.enable_register_route {
        rocket::routes![login::login, logout::logout, register::register]
    } else {
        rocket::routes![login::login, logout::logout]
    }
}
