use radmin::rocket::http::Status;
use radmin::serde_json::json;

use radmin::acl::factories::{AccountFactory, RoleFactory};

use radmin::acl::{AclModule, AclModuleConfig};
use radmin::client::ApiClient;
use radmin::modules::Modules;

#[test]
fn simple_success() {
    let acl_config = AclModuleConfig::default().set_enable_crud("admin/");
    let mut modules = Modules::new();
    modules.add_module(AclModule::new(acl_config));
    let mut client = ApiClient::new(Some(modules)).expect("Failed to build test client");

    let admin_role = RoleFactory::default()
        .name("admin")
        .insert(client.db.as_ref());
    let account = AccountFactory::default()
        .roles(vec![admin_role.id])
        .set_password("password")
        .expect("Failed to set account password")
        .insert(client.db.as_ref());

    let items = vec![json!(admin_role)];

    client.acting_as("password", account);

    let mut response = client.get("/crud/admin/roles/tableData").dispatch();
    let res_data = json!({
        "items": items,
        "total": 1,
        "total_pages": 1,
        "page": 1,
        "per_page": 10
    });

    assert_eq!(Some(json!(res_data).to_string()), response.body_string());
}

#[test]
fn unauthorized() {
    let acl_config = AclModuleConfig::default().set_enable_crud("admin/");
    let mut modules = Modules::new();
    modules.add_module(AclModule::new(acl_config));
    let mut client = ApiClient::new(Some(modules)).expect("Failed to build test client");

    let admin_role = RoleFactory::default()
        .name("other")
        .insert(client.db.as_ref());
    let account = AccountFactory::default()
        .roles(vec![admin_role.id])
        .set_password("password")
        .expect("Failed to set account password")
        .insert(client.db.as_ref());

    client.acting_as("password", account);

    assert_eq!(
        Status::Unauthorized,
        client
            .get("/crud/admin/roles/tableData")
            .dispatch()
            .status()
    );
}
