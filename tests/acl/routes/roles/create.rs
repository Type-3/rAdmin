use radmin::rocket::http::Status;
use radmin::rocket_contrib::json;

use radmin::acl::factories::{AccountFactory, PermissionFactory, RoleFactory};
use radmin::acl::traits::{HasPermissions, HasRoles};
use radmin::client::ApiClient;

#[test]
fn simple_success() {
    let mut client = ApiClient::new(None).expect("Failed to build test client");

    let account = AccountFactory::default()
        .set_password("password")
        .expect("Failed to set account password")
        .insert(client.db.as_ref());

    PermissionFactory::default()
        .name("admin.roles.modify")
        .insert(client.db.as_ref());

    let role1 = RoleFactory::default().insert(client.db.as_ref());
    role1
        .assign_permission_name("admin.roles.modify", client.db.as_ref())
        .unwrap();
    account
        .assign_role_id(role1.id, client.db.as_ref())
        .unwrap();

    client.acting_as("password", account);

    let response = client
        .post("/api/admin/roles/create")
        .body(
            json!({
                "name": "new_role",
                "label": "label",
                "description": "description",
                "permissions": []
            })
            .to_string(),
        )
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}
