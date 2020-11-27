use radmin::rocket::http::Status;
use radmin::serde_json::json;

use radmin::acl::factories::AccountFactory;
use radmin::client::ApiClient;

#[test]
fn simple_success() {
    let mut client = ApiClient::new(None).expect("Failed to build test client");

    let account = AccountFactory::default()
        .set_password("password")
        .expect("Failed to set account password")
        .insert(client.db.as_ref());

    assert_eq!(
        client
            .post("/api/auth/login")
            .body(json!({"identifier": account.username, "password": "password"}).to_string())
            .dispatch()
            .status(),
        Status::Ok
    );

    client.acting_as("password", account.clone());

    client.assert_logged_in(&account);
}

#[test]
fn failure() {
    let mut client = ApiClient::new(None).expect("Failed to build test client");

    let account = AccountFactory::default()
        .set_password("password")
        .expect("Failed to set account password")
        .insert(client.db.as_ref());

    assert_eq!(
        client
            .post("/api/auth/login")
            .body(json!({"identifier": account.username, "password": "pass"}).to_string())
            .dispatch()
            .status(),
        Status::Unauthorized
    );

    client.assert_logged_out(&account);
}
