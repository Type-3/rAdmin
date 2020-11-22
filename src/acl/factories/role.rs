use diesel::{PgConnection, RunQueryDsl};
use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};

use crate::acl::models::Role;
use crate::acl::schema::roles;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Insertable, Dummy)]
#[table_name = "roles"]
pub struct RoleFactory {
    #[dummy("Word()")]
    pub name: String,
    #[dummy("Words(2..4)")]
    pub label: Option<String>,
    #[dummy("Sentence()")]
    pub description: Option<String>,
    pub is_super: bool,
}

impl RoleFactory {
    pub fn name<S: Into<String>>(mut self, name: S) -> RoleFactory {
        self.name = name.into();
        self
    }
    pub fn label<S: Into<Option<String>>>(mut self, label: S) -> RoleFactory {
        self.label = label.into();
        self
    }
    pub fn description<S: Into<Option<String>>>(mut self, description: S) -> RoleFactory {
        self.description = description.into();
        self
    }

    pub fn is_super(mut self, b: bool) -> RoleFactory {
        self.is_super = b;
        self
    }

    pub fn insert(self, conn: &PgConnection) -> Role {
        diesel::insert_into(crate::acl::schema::roles::table)
            .values(&self)
            .get_result(conn)
            .expect(&format!("Failed to insert into database: {:?}", &self))
    }
}

impl Default for RoleFactory {
    fn default() -> RoleFactory {
        let mut factory: RoleFactory = Faker.fake();
        factory.is_super = false;
        factory
    }
}
