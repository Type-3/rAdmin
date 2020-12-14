use crate::acl::AclModuleConfig;

mod login;
pub mod logout;
pub mod register;

pub fn api_routes(config: &AclModuleConfig) -> Vec<rocket::Route> {
    if config.enable_api_routes {
        if config.enable_register_route {
            rocket::routes![login::api_login, logout::api_logout, register::api_register]
        } else {
            rocket::routes![login::api_login, logout::api_logout]
        }
    } else {
        vec![]
    }
}

pub fn form_routes(config: &AclModuleConfig) -> Vec<rocket::Route> {
    if config.enable_form_routes {
        if config.enable_register_route {
            rocket::routes![login::form_login, logout::logout, register::form_register]
        } else {
            rocket::routes![login::form_login, logout::logout]
        }
    } else {
        vec![]
    }
}