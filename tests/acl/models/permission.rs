use diesel::RunQueryDsl;

use radmin::acl::factories::PermissionFactory;
use radmin::acl::models::Permission;
use radmin::acl::schema::permissions;
use radmin::client::DbClient;

#[test]
fn simple_insert() {
    let conn = DbClient::new(None).unwrap();
    let permission = PermissionFactory::default().insert(conn.as_ref());
    conn.assert_table_has::<Permission, _>(permissions::table, permission.id, permission);
}

#[test]
fn simple_delete() {
    let conn = DbClient::new(None).unwrap();
    let permission = PermissionFactory::default().insert(conn.as_ref());
    conn.assert_table_has(permissions::table, permission.id, permission.clone());
    diesel::delete(&permission).execute(conn.as_ref()).unwrap();
    conn.assert_table_missing::<Permission, _>(permissions::table, permission.id);
}
