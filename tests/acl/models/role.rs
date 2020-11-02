use diesel::RunQueryDsl;

use radmin::acl::factories::RoleFactory;
use radmin::acl::models::Role;
use radmin::acl::schema::roles;
use radmin::client::DbClient;

#[test]
fn simple_insert() {
    let conn = DbClient::new(None).unwrap();
    let role = RoleFactory::default().insert(conn.as_ref());
    conn.assert_table_has::<Role, _>(roles::table, role.id, role);
}

#[test]
fn simple_delete() {
    let conn = DbClient::new(None).unwrap();
    let role = RoleFactory::default().insert(conn.as_ref());
    conn.assert_table_has(roles::table, role.id, role.clone());
    diesel::delete(&role).execute(conn.as_ref()).unwrap();
    conn.assert_table_missing::<Role, _>(roles::table, role.id);
}
