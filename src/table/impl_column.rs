use crate::table::Column;


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
}