use radmin::rocket::http::Status;

use radmin::acl::factories::{AccountFactory, RoleFactory};
use radmin::acl::models::Role;
use radmin::acl::schema::roles;
use radmin::client::ApiClient;

#[test]
fn simple_success() {
    let mut client = ApiClient::new(None).expect("Failed to build test client");
    let admin_role = RoleFactory::default()
        .name("admin")
        .insert(client.db.as_ref());
    let account = AccountFactory::default()
        .roles(vec![admin_role.id])
        .set_password("password")
        .expect("Failed to set account password")
        .insert(client.db.as_ref());

    let role2 = RoleFactory::default().insert(client.db.as_ref());

    client.acting_as("password", account);
    let route = format!("/api/admin/roles/{}", role2.id);

    assert_eq!(client.delete(&route).dispatch().status(), Status::Ok);
    client
        .db
        .assert_table_missing::<Role, _>(roles::table, role2.id);
}
