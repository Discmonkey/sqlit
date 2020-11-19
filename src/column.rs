use Vec;
use std::cmp::Ordering;


type Date = i64;

pub enum Column {
    Floats(Vec<f64>),
    Ints(Vec<i64>),
    Strings(Vec<String>),
    Booleans(Vec<bool>),
    Dates(Vec<Date>), // lets just unix timestamps, for now
    Ordering(Vec<Ordering>),
}

pub enum ColumnValue {
    Float(f64),
    Int(i64),
    String(String),
    Boolean(bool),
    Date(Date),
}
