pub mod accounts;
pub mod auth;
pub mod permissions;
pub mod roles;

use crate::modules::RoutesModule;

pub struct AclRoutesMod;

impl RoutesModule for AclRoutesMod {
    fn routes(&self) -> Vec<(String, Vec<rocket::Route>)> {
        vec![
            ("auth".into(), auth::api_routes()),
            ("admin/roles".into(), roles::api_routes()),
            ("admin/permissions".into(), permissions::api_routes()),
            ("admin/accounts".into(), accounts::api_routes()),
        ]
    }
}
