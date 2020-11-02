use radmin::rocket::http::Status;
use radmin::rocket_contrib::json;

use radmin::acl::factories::{AccountFactory, PermissionFactory};
use radmin::acl::traits::HasPermissions;

use radmin::client::ApiClient;

#[test]
fn simple_success() {
    let mut client = ApiClient::new(None).expect("Failed to build test client");

    let account = AccountFactory::default()
        .set_password("password")
        .expect("Failed to set account password")
        .insert(client.db.as_ref());

    let permission = PermissionFactory::default()
        .name("admin.permissions.modify")
        .insert(client.db.as_ref());

    account
        .assign_permission_id(permission.id, client.db.as_ref())
        .unwrap();

    client.acting_as("password", account);

    let response = client
        .post("/api/admin/permissions/create")
        .body(
            json!({
                "name": "new_permission",
                "label": "label",
                "description": "description"
            })
            .to_string(),
        )
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}
