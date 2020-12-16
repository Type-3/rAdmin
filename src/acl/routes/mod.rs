pub mod accounts;
pub mod auth;
pub mod roles;

mod avatar;

use crate::acl::AclModuleConfig;
use crate::modules::RoutesModule;

pub struct AclRoutesMod(AclModuleConfig);

impl AclRoutesMod {
    pub(crate) fn new(config: AclModuleConfig) -> AclRoutesMod {
        AclRoutesMod(config)
    }
}

impl RoutesModule for AclRoutesMod {
    fn routes(&self) -> Vec<(String, Vec<rocket::Route>)> {
        let mut routes = vec![
            ("api/auth".into(), auth::api_routes(&self.0)),
            ("auth".into(), auth::form_routes(&self.0)),
            ("avatars".into(), rocket::routes![avatar::avatar_image]),
        ];
        if let Some(route) = &self.0.enable_crud_routes {
            routes.push((format!("{}roles", route), roles::api_routes()));
            routes.push((format!("{}accounts", route), accounts::api_routes()));
        }
        if let Some(route) = &self.0.enable_form_routes {
            routes.push((format!("{}accounts", route), accounts::form_routes()));
            routes.push((format!("{}roles", route), roles::form_routes()));
        }
        routes
    }
}
