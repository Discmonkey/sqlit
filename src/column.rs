use Vec;


type Date = i64;

pub enum Column {
    Floats(Vec<f64>),
    Ints(Vec<i64>),
    Strings(Vec<String>),
    Booleans(Vec<bool>),
    Dates(Vec<Date>), // lets just unix timestamps, for now
}

pub enum ColumnValue {
    Float(f64),
    Int(i64),
    String(String),
    Boolean(bool),
    Date(Date),
}


impl Column {
    pub fn at(&self, idx: usize) -> ColumnValue {
        match self {
            Self::Floats(s) => ColumnValue::Float(s[idx]),
            Self::Ints(i) => ColumnValue::Int(i[idx]),
            Self::Strings(s) => ColumnValue::String(s[idx].clone()),
            Self::Booleans(b) => ColumnValue::Boolean(b[idx]),
            Self::Dates(d) => ColumnValue::Date(d[idx]),
        }
    }
}