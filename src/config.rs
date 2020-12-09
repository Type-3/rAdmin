use std::collections::HashMap;
use std::env;

use rocket::config::{Config, ConfigError, Environment, Value};

fn testing_config() -> Result<Config, ConfigError> {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(env::var("DATABASE_URL").unwrap()));
    databases.insert("radmin_db", Value::from(database_config));

    let storage_path = env::var("STORAGE_PATH").unwrap_or_else(|_| "data".into());

    let mut config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .port(5000)
        .extra("databases", databases)
        .extra("storage_path", Value::String(storage_path));

    if let Ok(path) = env::var("TEMPLATE_PATH") {
        config = config.extra("template_dir", path);
    }

    Ok(config.finalize()?)
}

fn local_config() -> Result<Config, ConfigError> {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(env::var("DATABASE_URL").unwrap()));
    databases.insert("radmin_db", Value::from(database_config));

    let storage_path = env::var("STORAGE_PATH").unwrap_or_else(|_| "data".into());

    let mut config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(5000)
        .extra("databases", databases)
        .extra("storage_path", storage_path);

    if cfg!(feature = "tera") || cfg!(feature = "handlebars") {
        if let Ok(path) = env::var("TEMPLATE_PATH") {
            config = config.extra("template_dir", path);
        }
    }

    Ok(config.finalize()?)
}

fn development_config() -> Result<Config, ConfigError> {
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(env::var("DATABASE_URL").unwrap()));
    databases.insert("radmin_db", Value::from(database_config));

    let storage_path = env::var("STORAGE_PATH").unwrap_or_else(|_| "data".into());

    let mut config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(5000)
        .extra("databases", databases)
        .extra("storage_path", storage_path);

    if cfg!(feature = "tera") || cfg!(feature = "handlebars") {
        if let Ok(path) = env::var("TEMPLATE_PATH") {
            config = config.extra("template_dir", path);
        }
    }
    Ok(config.finalize()?)
}

/// Return a tuple of an app-specific config and a Rocket config.
pub fn get_rocket_config(conf_name: Option<&str>) -> Result<Config, ConfigError> {
    dotenv::dotenv().unwrap();
    let config = conf_name
        .map(|item| item.to_string())
        .unwrap_or_else(|| env::var("ENVIRONMENT").unwrap_or_else(|_| "development".into()));
    match config.as_ref() {
        "testing" => testing_config(),
        "local" => local_config(),
        "development" => development_config(),
        config_name => Err(ConfigError::BadEnv(format!(
            "No valid config chosen: {}",
            config_name
        ))),
    }
}
