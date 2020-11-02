use diesel::RunQueryDsl;

use radmin::acl::factories::AccountFactory;
use radmin::acl::models::Account;
use radmin::acl::schema::accounts;
use radmin::client::DbClient;

#[test]
fn simple_insert() {
    let conn = DbClient::new(None).unwrap();
    let account = AccountFactory::default().insert(conn.as_ref());
    conn.assert_table_has::<Account, _>(accounts::table, account.id, account);
}

#[test]
fn simple_delete() {
    let conn = DbClient::new(None).unwrap();
    let account = AccountFactory::default().insert(conn.as_ref());
    conn.assert_table_has(accounts::table, account.id, account.clone());
    diesel::delete(&account).execute(conn.as_ref()).unwrap();
    conn.assert_table_missing::<Account, _>(accounts::table, account.id);
}
