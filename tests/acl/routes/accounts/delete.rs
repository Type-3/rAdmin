use radmin::rocket::http::Status;

use radmin::acl::factories::{AccountFactory, PermissionFactory};
use radmin::acl::models::Account;
use radmin::acl::schema::accounts;
use radmin::acl::traits::HasPermissions;

use radmin::client::ApiClient;

#[test]
fn simple_success() {
    let mut client = ApiClient::new(None).expect("Failed to build test client");

    let account = AccountFactory::default()
        .set_password("password")
        .expect("Failed to set account password")
        .insert(client.db.as_ref());

    let account2 = AccountFactory::default().insert(client.db.as_ref());

    PermissionFactory::default()
        .name("admin.accounts.delete")
        .insert(client.db.as_ref());

    account
        .assign_permission_name("admin.accounts.delete", client.db.as_ref())
        .unwrap();

    client.acting_as("password", account);
    let route = format!("/api/admin/accounts/{}", account2.id);

    let status = client.delete(&route).dispatch();

    assert_eq!(status.status(), Status::Ok);
    client
        .db
        .assert_table_missing::<Account, _>(accounts::table, account2.id);
}
