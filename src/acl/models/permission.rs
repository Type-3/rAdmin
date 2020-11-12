use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::acl::schema::permissions;

#[derive(Debug, PartialEq, Clone, Queryable, Serialize, Deserialize, Identifiable, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

use cli_table::Cell;

impl Into<Vec<Cell>> for Permission {
    fn into(self) -> Vec<Cell> {
        let label = self.label.unwrap_or_else(|| "None".into());
        let description = self.description.unwrap_or_else(|| "None".into());
        vec![
            Cell::new(&self.id, Default::default()),
            Cell::new(&self.name, Default::default()),
            Cell::new(&label, Default::default()),
            Cell::new(&description, Default::default()),
        ]
    }
}
