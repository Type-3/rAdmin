use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::{establish_connection, radmin_global_search};
use crate::ServerError;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: Uuid,
    pub permission: String,
    pub context: String,
    pub fields: Vec<(String, String)>,
}

pub type SearchConfig = Vec<(String, String, String, Vec<String>)>;

pub struct GlobalSearch(SearchConfig);

impl GlobalSearch {
    pub fn new(modules: SearchConfig) -> GlobalSearch {
        GlobalSearch(modules)
    }

    pub fn query(&self, query: &str) -> Result<Vec<SearchResult>, ServerError> {
        use diesel::RunQueryDsl;
        let conn = establish_connection()?;
        let schema = self.get_schema();
        let tables = Some(self.get_query_tables()?);
        let output: Vec<(String, String, String, uuid::Uuid, String)> =
            diesel::select(radmin_global_search(query, tables, schema)).load(&conn)?;
        Ok(self.handle_results(output)?)
    }

    fn get_query_tables(&self) -> Result<Vec<&String>, ServerError> {
        let mut tables = vec![];
        for (x, _, _, _) in &self.0 {
            tables.push(x);
        }
        Ok(tables)
    }

    fn get_schema(&self) -> Option<&'static str> {
        Some("public")
    }

    fn handle_results(
        &self,
        response: Vec<(String, String, String, uuid::Uuid, String)>,
    ) -> Result<Vec<SearchResult>, ServerError> {
        let mut results: Vec<SearchResult> = vec![];
        for (_schema, table, column, id, value) in response {
            'query: for (table_name, context, permission, columns) in &self.0 {
                if table_name == &table && columns.contains(&column) {
                    let mut already_inserted = false;
                    for result in &mut results {
                        if result.id == id && &result.context == context {
                            already_inserted = true;
                            if !result.fields.contains(&(column.clone(), value.clone())) {
                                result.fields.push((column.clone(), value.clone()));
                                continue 'query;
                            }
                        }
                    }
                    if !already_inserted {
                        let res = SearchResult {
                            id,
                            context: context.clone(),
                            permission: permission.clone(),
                            fields: vec![(column.clone(), value.clone())],
                        };
                        results.push(res);
                    }
                }
            }
        }
        Ok(results)
    }
}
