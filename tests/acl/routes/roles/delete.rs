use radmin::rocket::http::Status;

use radmin::acl::factories::{AccountFactory, PermissionFactory, RoleFactory};
use radmin::acl::models::Role;
use radmin::acl::schema::roles;
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
        .name("admin.roles.delete")
        .insert(client.db.as_ref());

    let role1 = RoleFactory::default().insert(client.db.as_ref());
    let role2 = RoleFactory::default().insert(client.db.as_ref());

    role1
        .assign_permission_name("admin.roles.delete", client.db.as_ref())
        .unwrap();
    account
        .assign_role_id(role1.id, client.db.as_ref())
        .unwrap();

    client.acting_as("password", account);
    let route = format!("/api/admin/roles/{}", role2.id);

    assert_eq!(client.delete(&route).dispatch().status(), Status::Ok);
    client
        .db
        .assert_table_missing::<Role, _>(roles::table, role2.id);
}
