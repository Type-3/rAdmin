use diesel::PgConnection;
use serde::Deserialize;

use crate::acl::factories::PermissionFactory;
use crate::acl::requests::PermissionRequest;
use crate::traits::Submitable;
use crate::ServerError;

#[radmin_macros::from_similar(PermissionRequest)]
#[derive(Deserialize, Debug)]
pub struct PermissionCreateForm {
    pub name: String,
    pub label: Option<String>,
    pub description: Option<String>,
}

impl Submitable for PermissionCreateForm {
    fn submit(self, conn: &PgConnection) -> Result<(), ServerError> {
        PermissionFactory::default()
            .name(self.name)
            .label(self.label)
            .description(self.description)
            .insert(conn);
        Ok(())
    }
}
