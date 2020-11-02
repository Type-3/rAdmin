use clap::{crate_authors, crate_description, crate_name, crate_version};

use crate::modules::ServerModule;

pub struct Application {
    pub(crate) name: &'static str,
    pub(crate) version: &'static str,
    pub(crate) author: &'static str,
    pub(crate) description: &'static str,
    pub(crate) modules: crate::modules::Modules,
}

impl Default for Application {
    fn default() -> Application {
        Self::new()
    }
}

impl Application {
    pub fn new() -> Application {
        Application {
            name: crate_name!(),
            version: crate_version!(),
            author: crate_authors!(),
            description: crate_description!(),
            modules: crate::modules::Modules::default(),
        }
    }

    pub fn name(mut self, name: &'static str) -> Application {
        self.name = name;
        self
    }

    pub fn version(mut self, version: &'static str) -> Application {
        self.version = version;
        self
    }

    pub fn author(mut self, author: &'static str) -> Application {
        self.author = author;
        self
    }

    pub fn description(mut self, description: &'static str) -> Application {
        self.description = description;
        self
    }

    pub fn add_module<T: ServerModule + 'static>(mut self, module: T) -> Application {
        self.modules.add_module(module);
        self
    }

    pub fn add_module_default<T: ServerModule + Default + 'static>(mut self) -> Application {
        self.modules.add_module_default::<T>();
        self
    }
}
