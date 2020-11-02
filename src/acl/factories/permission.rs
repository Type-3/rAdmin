use diesel::PgConnection;
use diesel_factories::Factory;
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::acl::models::Permission;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Factory, Dummy)]
#[factory(
    model = "Permission",
    table = "crate::acl::schema::permissions",
    id = "Uuid"
)]
pub struct PermissionFactory {
    #[dummy("Word()")]
    pub name: String,
    #[dummy("Words(2..4)")]
    pub label: Option<String>,
    #[dummy("Sentence()")]
    pub description: Option<String>,
}

impl PermissionFactory {
    pub fn insert(self, conn: &PgConnection) -> Permission {
        Factory::insert(self, conn)
    }
}

impl Default for PermissionFactory {
    fn default() -> PermissionFactory {
        Faker.fake()
    }
}
