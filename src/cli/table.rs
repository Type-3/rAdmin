use crate::traits::Paginate;

use clap::{value_t, ArgMatches};
use diesel::associations::HasTable;
use diesel::pg::Pg;
use diesel::query_builder::{AsQuery, QueryFragment, QueryId};
use diesel::types::HasSqlType;
use diesel::{PgConnection, Queryable};
use paginator::Paginator;

use cli_table::{
    format::{CellFormat, Color, TableFormat},
    Cell, Row, Table as CliTable,
};

/// A generic Command line table that maps to a diesel table row.
pub trait Table<M>: Default
where
    M: HasTable + Into<Vec<Cell>>,
    M: Queryable<<<M as HasTable>::Table as AsQuery>::SqlType, Pg>,
    Pg: HasSqlType<<<M as HasTable>::Table as AsQuery>::SqlType>,
    <M as HasTable>::Table: QueryId,
    <<M as HasTable>::Table as AsQuery>::Query: QueryId + QueryFragment<Pg>,
{
    /// **Note** The number of headers must match the number
    /// of columns returned from `Self::Model`
    const HEADERS: &'static [&'static str];

    /// Render to table
    fn display(&self, options: Option<&ArgMatches>) {
        let conn = crate::establish_connection().expect("Failed to connect to database");

        let (page, per_page) = {
            if let Some(args) = options {
                (
                    value_t!(args.value_of("page"), i64).ok(),
                    value_t!(args.value_of("per_page"), i64).ok(),
                )
            } else {
                (None, None)
            }
        };

        let (data, _total, pages, page, _per) = M::table()
            .as_query()
            .paginate(page)
            .per_page(per_page)
            .load_and_count_pages::<M, PgConnection>(&conn)
            .unwrap();

        let mut data: Vec<Vec<Cell>> = data.into_iter().map(|item| item.into()).collect();

        let headers = Self::HEADERS
            .iter()
            .map(|item| Cell::new(item, self.header_format()))
            .collect();

        data.insert(0, headers);

        if pages > 1 {
            let paginator = Paginator::builder(pages as usize)
                .current_page(page as usize)
                .build_paginator()
                .unwrap()
                .paginate();

            let items = paginator::page_items_to_string(paginator.as_slice());

            let mut page_row_data = vec![Cell::new(&items, Default::default())];

            for _ in 0..data[0].len() - 1 {
                page_row_data.push(Cell::new("", Default::default()));
            }

            data.push(page_row_data);
        }

        CliTable::new(
            data.into_iter().map(Row::new).collect(),
            Self::table_config(),
        )
        .unwrap()
        .print_stdout()
        .unwrap();
    }

    fn header_format(&self) -> CellFormat {
        CellFormat::builder()
            .foreground_color(Some(Color::Yellow))
            .bold(true)
            .build()
    }

    fn table_config() -> TableFormat {
        TableFormat::default().foreground(Color::Red)
    }
}
