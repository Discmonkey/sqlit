mod impl_table;
mod impl_store;
mod impl_table_display;
mod impl_column;

use std::collections::HashMap;

pub type DateTime = i64;

#[derive(Clone, Debug)]
pub enum ColumnType {
    Float,
    Int,
    String,
    Boolean,
    Date,
}

#[derive(Clone, Debug)]
pub enum Column {
    Floats(Vec<f64>),
    Ints(Vec<i64>),
    Strings(Vec<String>),
    Booleans(Vec<bool>),
    Dates(Vec<DateTime>), // lets just unix timestamps, for now
}


#[derive(Clone, Debug)]
pub struct NamedColumn {
    pub name: String,
    pub column: Column
}

#[derive(Clone)]
pub struct Table {
    alias: String,
    column_names: Vec<String>, // list of columns names
    column_map: HashMap<(String, String), usize>, // a map of (table, column name) to indices
    columns: Vec<Column>, // the actual data
    num_rows: usize, // number of rows in the table
}

#[derive(Clone)]
pub struct TableMeta {
    pub columns: Vec<(String, ColumnType)>,
    pub length: usize,
    pub alias: String,
}

pub struct Store {
    tables: HashMap <String, Table>
}
