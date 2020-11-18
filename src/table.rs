use std::collections::HashMap;
use crate::column::Column;

pub struct Table {
    columns: HashMap<String, usize>,
    values: Vec<Column>,
    rows: i64,
}

impl Table {

    pub fn row(&self, idx: usize) {

    }

}
