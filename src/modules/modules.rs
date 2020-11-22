use super::ServerModule;

pub struct Modules(pub(crate) Vec<Box<dyn ServerModule>>);

impl Default for Modules {
    fn default() -> Modules {
        Modules(vec![Box::new(crate::acl::AclModule)])
    }
}

impl Modules {
    pub fn add_module<T: ServerModule + 'static>(&mut self, module: T) {
        self.0.push(Box::new(module));
    }

    pub fn add_module_default<T: ServerModule + Default + 'static>(&mut self) {
        self.0.push(Box::new(T::default()));
    }
}
