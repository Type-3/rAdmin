use radmin::serde_json::json;

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
        .name("admin.permissions.list")
        .insert(client.db.as_ref());

    account
        .assign_permission_id(permission.id, client.db.as_ref())
        .expect("Failed to assign permission");

    client.acting_as("password", account);

    let mut response = client.get("/api/admin/permissions/tableData").dispatch();

    let items = vec![json!(permission)];

    let res_data = json!({
        "items": items,
        "total": 1,
        "total_pages": 1,
        "page": 1,
        "per_page": 10
    });

    assert_eq!(Some(json!(res_data).to_string()), response.body_string());
}
