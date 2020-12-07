use radmin::rocket::http::Status;

use radmin::acl::factories::{AccountFactory, RoleFactory};
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
    client.acting_as("password", account.clone());

    assert_eq!(
        client.post("/crud/auth/logout").dispatch().status(),
        Status::Ok
    );

    client.assert_logged_out(&account);
}

#[test]
fn unauthorized() {
    let mut client = ApiClient::new(None).expect("Failed to build test client");

    AccountFactory::default()
        .set_password("password")
        .expect("Failed to set account password")
        .insert(client.db.as_ref());

    assert_eq!(
        client.post("/crud/auth/logout").dispatch().status(),
        Status::Unauthorized
    );
}
