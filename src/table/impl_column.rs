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

impl Column {
    pub fn limit(&mut self, length: usize) {
        apply!(self, truncate, length);
    }

    pub fn len(&self) -> usize {
        apply!(self, len,)
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