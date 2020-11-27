use radmin::rocket::http::Status;
use radmin::serde_json::json;

use radmin::client::ApiClient;
use radmin::acl::{AclModuleConfig, AclModule};
use radmin::modules::Modules;

#[test]
fn simple_success() {
    let acl_config = AclModuleConfig::default().set_enable_register_route(true);
    let mut modules = Modules::new();
    modules.add_module(AclModule::new(acl_config));
    let mut client = ApiClient::new(Some(modules))
        .expect("Failed to build test client");

    assert_eq!(
        client
            .post("/api/auth/register")
            .body(json!({"email": "some@email.com", "username": "newUsername", "password": "newPassword", "password_config": "newPassword"}).to_string())
            .dispatch()
            .status(),
        Status::Ok
    );
}
