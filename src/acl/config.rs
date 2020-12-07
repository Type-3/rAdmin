#[derive(Default, Clone)]
pub struct AclModuleConfig {
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
}
