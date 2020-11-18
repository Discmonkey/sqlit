use crate::table::Table;
use std::cmp::Ordering;
use crate::scalar::Scalar;

enum Statement {
    Scalar(Scalar),
    Identifier(String)
}

struct Column {
    name: Option<String>,
    value: Statement,
}

trait TableWriter {
    fn write(&self, table: Table);
}

pub struct Select {
    columns: Vec<Column>,
    from: Table,
    where_clause: Option<Box<dyn Fn(Table, usize) -> bool>>,
    group_by_clause: Option<Vec<String>>,
    order_by_clause: Option<Box<dyn Fn(Table, usize) -> Ordering>>,
    into_clause: Option<Box<dyn TableWriter>>,
}