use crate::table::{Column, ColumnType};
use std::cmp::Ordering;


macro_rules! apply {
    ($self: expr, $method: tt, $($arg:expr),*) => {
        match $self {
            Column::Booleans(b) => b.$method($($arg,)*),
            Column::Dates(d) => d.$method($($arg,)*),
            Column::Floats(f) => f.$method($($arg,)*),
            Column::Ints(i) => i.$method($($arg,)*),
            Column::Strings(s) => s.$method($($arg,)*),
        }
    }
}

fn select<T: Clone>(values: &Vec<T>, selections: &Vec<bool>) -> Vec<T> {
    values.into_iter().zip(selections.iter()).filter_map(|(val, s)| {
        if *s {
            Some(val.clone())
        } else {
            None
        }
    }).collect()
}

impl Column {
    pub fn limit(&mut self, length: usize) {
        apply!(self, truncate, length);
    }

    pub fn len(&self) -> usize {
        apply!(self, len,)
    }


    pub fn select(&self, selections: &Vec<bool>) -> Self {
        match self {
            Column::Booleans(v) => Column::Booleans(select(v, selections)),
            Column::Ints(v) => Column::Ints(select(v, selections)),
            Column::Floats(v) => Column::Floats(select(v, selections)),
            Column::Strings(v) => Column::Strings(select(v, selections)),
            Column::Dates(v) => Column::Dates(select(v, selections)),
        }
    }

    pub fn type_(&self) -> ColumnType {
        match self {
            Column::Booleans(_) => ColumnType::Boolean,
            Column::Ints(_) => ColumnType::Int,
            Column::Floats(_) => ColumnType::Float,
            Column::Dates(_) => ColumnType::Date,
            Column::Strings(_) => ColumnType::String
        }
    }

    pub fn elem_order(&self, i1: usize, i2: usize) -> Ordering {
        if i1 > self.len() || i2 > self.len() {
            return Ordering::Equal;
        }

        match self {
            Column::Booleans(b) => {
                if b[i1] == b[i2] {
                    Ordering::Equal
                } else if b[i1] {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            },

            Column::Ints(i) => i[i1].cmp(&i[i2]),
            Column::Floats(f) => f[i1].partial_cmp(&f[i2]).unwrap(),
            Column::Dates(d) => d[i1].cmp(&d[i2]),
            Column::Strings(s) => s[i1].cmp(&s[i2])
        }
    }

}