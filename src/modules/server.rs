use super::{
    CliModule, DatabaseModule, NullCliModule, NullDatabaseModule, NullRoutesModule, RoutesModule
};

pub trait ServerModule {
    fn identifier(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn config(&self, config: rocket::Config) -> rocket::Config {
        config
    }
    fn storage(&self) -> Vec<(&'static str, Vec<&'static str>)> {
        vec![]
    }
    fn cli(&self) -> Box<dyn CliModule> {
        Box::new(NullCliModule)
    }
    fn database(&self) -> Box<dyn DatabaseModule> {
        Box::new(NullDatabaseModule)
    }
    fn routes(&self) -> Box<dyn RoutesModule> {
        Box::new(NullRoutesModule)
    }
}
