use crate::table::ApiTable;
use crate::ServerError;
use diesel::{PgConnection, TextExpressionMethods, QueryDsl};
use crate::roles::AdminRole;
use crate::traits::Paginate;
use crate::acl::schema::roles;
use crate::acl::models::Role;

pub struct RolesTable;

impl ApiTable for RolesTable {
    type Role = AdminRole;
    type Model = Role;

    fn query(conn: &PgConnection, query: Option<String>, page: Option<i64>, per_page: Option<i64>) -> Result<(Vec<Self::Model>, i64, i64, i64, i64), ServerError> {
        if let Some(query) = query {
            Ok(roles::table
                .filter(roles::name.like(format!("%{}%", &query)))
                .or_filter(roles::description.like(format!("%{}%", &query)))
                .or_filter(roles::label.like(format!("%{}%", &query)))
                .paginate(page)
                .per_page(per_page)
                .load_and_count_pages::<Role, PgConnection>(conn)?)
        } else {
            Ok(roles::table
                .select(roles::all_columns)
                .paginate(page)
                .per_page(per_page)
                .load_and_count_pages::<Role, PgConnection>(conn)?)
        }
    }
}