#[derive(Clone)]
pub struct AclModuleConfig {
    pub enable_form_routes: bool,
    pub enable_api_routes: bool,
    pub enable_register_route: bool,
    pub enable_crud: Option<String>,
}
impl AclModuleConfig {
    pub fn set_enable_crud<S: Into<String>>(mut self, s: S) -> AclModuleConfig {
        self.enable_crud = Some(s.into());
        self
    }

    pub fn set_enable_register_route(mut self, b: bool) -> AclModuleConfig {
        self.enable_register_route = b;
        self
    }

    pub fn set_enable_api_routes(mut self, b: bool) -> AclModuleConfig {
        self.enable_api_routes = b;
        self
    }

    pub fn set_enable_form_routes(mut self, b: bool) -> AclModuleConfig {
        self.enable_form_routes = b;
        self
    }
}

impl Default for AclModuleConfig {
    fn default() -> AclModuleConfig {
        AclModuleConfig {
            enable_form_routes: true,
            enable_api_routes: true,
            enable_register_route: true,
            enable_crud: None
        }
    }
}