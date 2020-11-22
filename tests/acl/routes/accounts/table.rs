use diesel::QueryDsl;
use diesel::RunQueryDsl;
use radmin::serde_json::json;

use radmin::acl::factories::{AccountFactory, RoleFactory};
use radmin::acl::models::Account;
use radmin::acl::schema::accounts;
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

    let account_id = account.id;

    client.acting_as("password", account);

    let mut response = client.get("/api/admin/accounts/tableData").dispatch();

    let req_account: Account = accounts::table
        .find(&account_id)
        .first(client.db.as_ref())
        .unwrap();
    let items = vec![json!(req_account)];

    let res_data = json!({
        "items": items,
        "total": 1,
        "total_pages": 1,
        "page": 1,
        "per_page": 10
    });

    assert_eq!(Some(json!(res_data).to_string()), response.body_string());
}
