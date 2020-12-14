use diesel::{RunQueryDsl, PgConnection};

use crate::acl::models::Role;
use crate::acl::schema::roles;
use crate::roles::AdminRole;
use crate::ServerError;
use crate::select::{ApiSelect, SelectResult};


pub struct RoleSelect;

impl ApiSelect for RoleSelect {
    type Role = AdminRole;

    fn query(db: &PgConnection) -> Result<Vec<SelectResult>, ServerError> {
        Ok(roles::table.load(db)?
            .into_iter()
            .map(|item: Role| {
                let text = item.label.unwrap_or(item.name);
                SelectResult { id: text.clone(), text }
            })
            .collect::<Vec<SelectResult>>())
    }
}