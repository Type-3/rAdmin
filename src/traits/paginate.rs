use diesel::backend::Backend;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;

pub trait Paginate<T> {
    fn paginate(self, page: Option<i64>) -> Paginated<T>;
}

impl<T: diesel::query_builder::Query> Paginate<T> for T {
    fn paginate(self, page: Option<i64>) -> Paginated<Self> {
        Paginated {
            query: self,
            per_page: DEFAULT_PER_PAGE,
            page: page.unwrap_or(1),
        }
    }
}

const DEFAULT_PER_PAGE: i64 = 10;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
}

impl<T> Paginated<T> {
    pub fn per_page(self, per_page: Option<i64>) -> Self {
        let per_page = per_page.unwrap_or(10);
        Paginated { per_page, ..self }
    }

    pub fn load_and_count_pages<U, C>(self, conn: &C) -> QueryResult<(Vec<U>, i64, i64, i64, i64)>
    where
        C: diesel::Connection,
        Self: LoadQuery<C, (U, i64)>,
    {
        let page = self.page;
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok((records, total, total_pages, page, per_page))
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T, C: Connection> RunQueryDsl<C> for Paginated<T> {}

impl<T, B: Backend> QueryFragment<B> for Paginated<T>
where
    T: QueryFragment<B>,
{
    fn walk_ast(&self, mut out: AstPass<B>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        let offset = (self.page - 1) * self.per_page;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}
