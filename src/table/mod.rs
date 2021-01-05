mod impl_table;
mod impl_context;

use crate::column;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Table {
    alias: String,
    column_names: Vec<String>, // list of columns names
    column_map: HashMap<(String, String), usize>, // a map of (table, column name) to indices
    columns: Vec<column::Column>, // the actual data
    num_rows: usize, // number of rows in the table
}

pub struct Context {
    tables: HashMap <String, Table>
}