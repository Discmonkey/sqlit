use crate::table::{Column, ColumnType};


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
}