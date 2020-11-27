pub mod accounts;
pub mod auth;
pub mod roles;

mod avatar;

use crate::modules::RoutesModule;
use crate::acl::AclModuleConfig;

pub struct AclRoutesMod(AclModuleConfig);

impl AclRoutesMod {
    pub(crate) fn new(config: AclModuleConfig) -> AclRoutesMod {
        AclRoutesMod(config)
    }
}

impl RoutesModule for AclRoutesMod {
    fn routes(&self) -> Vec<(String, Vec<rocket::Route>)> {
        let mut routes = vec![
            ("auth".into(), auth::api_routes(&self.0)),
            ("avatars".into(), rocket::routes![avatar::avatar_image]),
        ];
        if self.0.enable_admin_crud {
            routes.push(("admin/roles".into(), roles::api_routes()));
            routes.push(("admin/accounts".into(), accounts::api_routes()));
        }
        routes
    }
}
