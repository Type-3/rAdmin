use radmin::rocket::http::Status;
use radmin::rocket_contrib::json;

use radmin::acl::factories::{AccountFactory, RoleFactory};
use radmin::client::ApiClient;
use radmin::acl::{AclModuleConfig, AclModule};
use radmin::modules::Modules;

#[test]
fn simple_success() {
    let acl_config = AclModuleConfig::default().set_enable_crud("admin/");
    let mut modules = Modules::new();
    modules.add_module(AclModule::new(acl_config));
    let mut client = ApiClient::new(Some(modules))
        .expect("Failed to build test client");

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
        .post("/api/admin/roles/create")
        .body(
            json!({
                "name": "new_role",
                "label": "label",
                "description": "description",
                "is_super": false
            })
            .to_string(),
        )
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}
