use radmin::rocket::http::Status;
use radmin::serde_json::json;

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
        .name("admin.roles.list")
        .insert(client.db.as_ref());

    let role1 = RoleFactory::default().insert(client.db.as_ref());
    role1
        .assign_permission_name("admin.roles.list", client.db.as_ref())
        .unwrap();
    account
        .assign_role_id(role1.id, client.db.as_ref())
        .unwrap();
    let items = vec![json!(role1)];

    client.acting_as("password", account);

    let mut response = client.get("/api/admin/roles/tableData").dispatch();
    let res_data = json!({
        "items": items,
        "total": 1,
        "total_pages": 1,
        "page": 1,
        "per_page": 10
    });

    assert_eq!(Some(json!(res_data).to_string()), response.body_string());
}

#[test]
fn unauthorized() {
    let mut client = ApiClient::new(None).expect("Failed to build test client");

    let account = AccountFactory::default()
        .set_password("password")
        .expect("Failed to set account password")
        .insert(client.db.as_ref());

    PermissionFactory::default()
        .name("admin.roles.list")
        .insert(client.db.as_ref());

    client.acting_as("password", account);

    assert_eq!(
        Status::Unauthorized,
        client.get("/api/admin/roles/tableData").dispatch().status()
    );
}
