use radmin::rocket::http::Status;
use radmin::serde_json::json;

use radmin::client::ApiClient;

#[test]
fn simple_success() {
    let mut client = ApiClient::new(None).expect("Failed to build test client");

    assert_eq!(
        client
            .post("/api/auth/register")
            .body(json!({"email": "some@email.com", "username": "newUsername", "password": "newPassword", "password_config": "newPassword"}).to_string())
            .dispatch()
            .status(),
        Status::Ok
    );
}
