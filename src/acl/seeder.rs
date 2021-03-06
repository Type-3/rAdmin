use crate::modules::Seeder;
use crate::traits::Submitable;
use crate::ServerError;
use clap::{Arg, ArgMatches};
use diesel::PgConnection;
use serde_json::{json, Map, Value};
use uuid::Uuid;

use std::path::PathBuf;

pub struct AclSeeder;

impl Seeder for AclSeeder {
    fn args(&self) -> Option<Vec<Arg<'static, 'static>>> {
        None
    }

    fn seed(&self, _matches: Option<&ArgMatches>, conn: &PgConnection) -> Result<(), ServerError> {
        use crate::acl::forms::{AccountCreateForm, RoleCreateForm};
        use std::fs::File;

        if let Ok(file) = File::open("seeds/roles.json") {
            let roles: Vec<RoleCreateForm> = serde_json::from_reader(file)?;
            for role in roles {
                role.submit(conn)?;
            }
        }
        if let Ok(file) = File::open("seeds/accounts.json") {
            let accounts: Vec<Map<String, Value>> = serde_json::from_reader(file)?;
            for mut account in accounts.into_iter() {
                account.insert("password_confirm".into(), account["password"].clone());
                if !account.contains_key("roles") {
                    account.insert("roles".into(), json!(Vec::<Value>::new()));
                }
                if account.contains_key("avatar") {
                    let mut seed_path = PathBuf::from("seeds/avatars/");
                    let mut out_path = PathBuf::from("data/avatars/");
                    let _avatar = account.remove("avatar").unwrap();
                    let avatar = _avatar.as_str().unwrap();
                    seed_path.push(&avatar);
                    if seed_path.exists() {
                        let id = Uuid::new_v4();
                        out_path.push(format!("{}.png", id));
                        std::fs::copy(&seed_path, &out_path)?;
                        account.insert("avatar".into(), json!(id));
                    }
                }
                let account: AccountCreateForm = serde_json::from_value(json!(account))?;
                account.submit(conn)?;
            }
        }
        Ok(())
    }
}
