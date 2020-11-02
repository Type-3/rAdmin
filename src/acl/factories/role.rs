use diesel::PgConnection;
use diesel_factories::Factory;
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::acl::models::Role;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Factory, Dummy)]
#[factory(model = "Role", table = "crate::acl::schema::roles", id = "Uuid")]
pub struct RoleFactory {
    #[dummy("Word()")]
    pub name: String,
    #[dummy("Words(2..4)")]
    pub label: Option<String>,
    #[dummy("Sentence()")]
    pub description: Option<String>,
}

impl RoleFactory {
    pub fn insert(self, conn: &PgConnection) -> Role {
        Factory::insert(self, conn)
    }
}

impl Default for RoleFactory {
    fn default() -> RoleFactory {
        Faker.fake()
    }
}
