use rocket::http::Status;
use rocket_contrib::json;

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

    client.acting_as("password", account);

    let response = client
        .post("/api/admin/accounts/create")
        .body(
            json!({
                "email": "username@email.com",
                "username": "username",
                "password": "password",
                "password_confirm": "password",
                "roles": []
            })
            .to_string(),
        )
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}
