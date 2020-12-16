use clap::{crate_authors, crate_description, crate_name, crate_version};

use crate::modules::ServerModule;

use std::sync::{Arc, Mutex};

pub struct Application {
    pub(crate) name: &'static str,
    pub(crate) version: &'static str,
    pub(crate) author: &'static str,
    pub(crate) description: &'static str,
    pub(crate) modules: crate::modules::Modules,
    pub(crate) configure: Arc<Mutex<dyn Fn(rocket::Rocket) -> rocket::Rocket>>
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
            configure: Arc::new(Mutex::new(|rocket: rocket::Rocket| {
                if cfg!(feature = "tera") || cfg!(feature = "handlebars") {
                    rocket.attach(rocket_contrib::templates::Template::custom(|engines| {
                        if cfg!(feature = "tera") {
                            engines.tera.register_filter("avatar", crate::template_helpers::tera::avatar);
                        }
                    }))
                } else {
                    rocket
                }
            }))
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

    pub fn modules(mut self, modules: crate::modules::Modules) -> Application {
        self.modules = modules;
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

    pub fn configure<F: Fn(rocket::Rocket) -> rocket::Rocket + 'static>(mut self, func: F) -> Application {
        self.configure = Arc::new(Mutex::new(func));
        self
    }
}
