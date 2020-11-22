use radmin::rocket::http::Status;
use radmin::serde_json::json;

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

    let items = vec![json!(admin_role)];

    client.acting_as("password", account);

    let mut response = client.get("/api/admin/roles/tableData").dispatch();
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
    let mut client = ApiClient::new(None).expect("Failed to build test client");

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
        client.get("/api/admin/roles/tableData").dispatch().status()
    );
}
