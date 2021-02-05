mod impl_table;
mod impl_store;
mod impl_table_display;
mod impl_column;
mod impl_table_hash;

use std::collections::HashMap;
use std::rc::Rc;

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
    Floats(Vec<Option<f64>>),
    Ints(Vec<Option<i64>>),
    Strings(Vec<Option<String>>),
    Booleans(Vec<Option<bool>>),
    Dates(Vec<Option<DateTime>>), // lets just unix timestamps, for now
}

#[derive(Clone, Debug)]
pub struct NamedColumn {
    pub name: String,
    pub column: Rc<Column>
}

#[derive(Clone)]
pub struct Table {
    alias: String,
    column_names: Vec<String>, // list of columns names
    column_map: HashMap<(String, String), usize>, // a map of (table, column name) to indices
    columns: Vec<Rc<Column>>, // the actual data
    limit: Option<usize>,
}

#[derive(Clone)]
pub struct TableMeta {
    pub columns: Vec<(String, ColumnType)>,
    pub length: usize,
    pub alias: String,
}

pub struct Store {
    tables: HashMap <String, Rc<Table>>
}
