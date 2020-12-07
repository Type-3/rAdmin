use crate::table::ApiTable;
use crate::ServerError;
use diesel::{PgConnection, TextExpressionMethods, QueryDsl};
use crate::roles::AdminRole;
use crate::traits::Paginate;
use crate::acl::schema::accounts;
use crate::acl::models::Account;

pub struct AccountsTable;

impl ApiTable for AccountsTable {
    type Role = AdminRole;
    type Model = Account;

    fn query(conn: &PgConnection, query: Option<String>, page: Option<i64>, per_page: Option<i64>) -> Result<(Vec<Self::Model>, i64, i64, i64, i64), ServerError> {
        if let Some(query) = query {
            Ok(accounts::table
                .filter(accounts::username.like(format!("%{}%", query)))
                .paginate(page)
                .per_page(per_page)
                .load_and_count_pages::<Account, PgConnection>(conn)?)
        } else {
            Ok(accounts::table
                .select(accounts::all_columns)
                .paginate(page)
                .per_page(per_page)
                .load_and_count_pages::<Account, PgConnection>(conn)?)
        }
    }
}
