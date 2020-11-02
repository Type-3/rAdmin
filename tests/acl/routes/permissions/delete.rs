use radmin::rocket::http::Status;

use radmin::acl::factories::{AccountFactory, PermissionFactory};
use radmin::acl::models::Permission;
use radmin::acl::schema::permissions;
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
        .name("admin.permissions.delete")
        .insert(client.db.as_ref());

    let permission2 = PermissionFactory::default()
        .name("random.permission")
        .insert(client.db.as_ref());

    account
        .assign_permission_id(permission.id, client.db.as_ref())
        .unwrap();

    client.acting_as("password", account);
    let route = format!("/api/admin/permissions/{}", permission2.id);

    assert_eq!(client.delete(&route).dispatch().status(), Status::Ok);
    client
        .db
        .assert_table_missing::<Permission, _>(permissions::table, permission2.id);
}
