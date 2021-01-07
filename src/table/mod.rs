mod impl_table;
mod impl_store;
mod impl_table_fmt;
mod impl_column;

use std::collections::HashMap;

pub type DateTime = i64;

#[derive(Clone, Debug)]
pub enum Column {
    Floats(Vec<f64>),
    Ints(Vec<i64>),
    Strings(Vec<String>),
    Booleans(Vec<bool>),
    Dates(Vec<DateTime>), // lets just unix timestamps, for now
}

#[derive(Clone)]
pub struct Table {
    alias: String,
    column_names: Vec<String>, // list of columns names
    column_map: HashMap<(String, String), usize>, // a map of (table, column name) to indices
    columns: Vec<Column>, // the actual data
    num_rows: usize, // number of rows in the table
}

pub struct Store {
    tables: HashMap <String, Table>
}
