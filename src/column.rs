use Vec;
use std::cmp::Ordering;


pub type DateTime = i64;

pub enum Column {
    Floats(Vec<f64>),
    Ints(Vec<i64>),
    Strings(Vec<String>),
    Booleans(Vec<bool>),
    Dates(Vec<DateTime>), // lets just unix timestamps, for now
    Ordering(Vec<Ordering>),
}

pub enum Value {
    Float(f64),
    Int(i64),
    String(String),
    Boolean(bool),
    DateTime(DateTime),
}

pub enum Type {
    Float,
    Int,
    String,
    Boolean,
    DateTime,
    Ordering // only used internally
}