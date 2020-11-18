use std::collections::HashMap;
use crate::column::{Column, ColumnValue};

pub struct Table {
    columns: HashMap<String, usize>,
    values: Vec<Column>,
    rows: usize,
}

impl Table {
    pub fn column(&self, which: &str) -> Option<&Column> {
        self.columns.get(which).map(|idx| {
            self.values[idx]
        })
    }
}
