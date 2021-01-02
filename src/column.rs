use Vec;

pub type DateTime = i64;

#[derive(Clone)]
pub enum Column {
    Floats(Vec<f64>),
    Ints(Vec<i64>),
    Strings(Vec<String>),
    Booleans(Vec<bool>),
    Dates(Vec<DateTime>), // lets just unix timestamps, for now
}