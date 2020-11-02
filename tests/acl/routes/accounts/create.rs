use rocket::http::Status;
use rocket_contrib::json;

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

    PermissionFactory::default()
        .name("admin.accounts.modify")
        .insert(client.db.as_ref());

    account
        .assign_permission_name("admin.accounts.modify", client.db.as_ref())
        .unwrap();

    client.acting_as("password", account);

    let response = client
        .post("/api/admin/accounts/create")
        .body(
            json!({
                "email": "username@email.com",
                "username": "username",
                "password": "password",
                "password_confirm": "password",
                "roles": [],
                "permissions": []
            })
            .to_string(),
        )
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}
