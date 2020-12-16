use radmin::rocket::http::Status;

use radmin::acl::factories::{AccountFactory, RoleFactory};
use radmin::acl::models::Account;
use radmin::acl::schema::accounts;

use radmin::acl::{AclModule, AclModuleConfig};
use radmin::client::ApiClient;
use radmin::modules::Modules;

#[test]
fn simple_success() {
    let acl_config = AclModuleConfig::default().set_enable_crud_routes("admin/");
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

    let account2 = AccountFactory::default()
        .set_password("password")
        .expect("Failed to set account password")
        .insert(client.db.as_ref());

    client.acting_as("password", account);
    let route = format!("/crud/admin/accounts/{}", account2.id);

    let status = client.delete(&route).dispatch();

    assert_eq!(status.status(), Status::Ok);
    client
        .db
        .assert_table_missing::<Account, _>(accounts::table, account2.id);
}
