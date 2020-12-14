use serde::{Deserialize, Serialize};

use crate::acl::schema::roles;

#[derive(Debug, PartialEq, Clone, Identifiable, Serialize, Deserialize, Queryable, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
#[primary_key(name)]
pub struct Role {
    pub name: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub is_super: bool
}

use cli_table::{CellStruct, Cell};

impl Into<Vec<CellStruct>> for Role {
    fn into(self) -> Vec<CellStruct> {
        let label = self.label.unwrap_or_else(|| "None".into());
        let description = self.description.unwrap_or_else(|| "None".into());
        vec![
            self.name.cell(),
            self.is_super.cell(),
            label.cell(),
            description.cell()
        ]
    }
}
