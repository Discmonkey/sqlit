use crate::table::Column;

impl Column {
    pub fn limit(&mut self, length: usize) {
        match self {
            Column::Booleans(b) => b.truncate(length),
            Column::Dates(d) => d.truncate(length),
            Column::Floats(f) => f.truncate(length),
            Column::Ints(i) => i.truncate(length),
            Column::Strings(s) => s.truncate(length)
        }
    }
}