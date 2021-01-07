use crate::table::Column;

pub enum Op {
    Add,
    Multiply,
    Subtract,
    Divide,
    And,
    Or,
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
    Concat,
    Split,
}

pub trait SqlOp {
    fn num_args() -> usize;
}

macro_rules! ok_op_bool {
    ($op: tt, $left: expr, $right: expr) => {
        Ok(Column::Booleans($left.iter().zip($right).map(|(&a, b)| {a $op b}).collect()))
    }
}

macro_rules! math_op_columns {
    ($op: tt, $left: expr, $right: expr) => {
        match ($left, $right) {
            (Column::Floats(floats_a), Column::Floats(floats_b)) => Ok(Column::Floats(floats_a.iter().zip(floats_b).map(|(a, b)| {a $op &b}).collect())),
            (Column::Floats(floats_a), Column::Ints(ints_b)) => Ok(Column::Floats(floats_a.iter().zip(ints_b).map(|(a, b)| {a $op (b as f64)}).collect())),
            (Column::Ints(ints_a), Column::Floats(floats_b)) => Ok(Column::Floats(ints_a.iter().zip(floats_b).map(|(&a, b)| {(a as f64) $op b}).collect())),
            (Column::Ints(ints_a), Column::Ints(ints_b)) => Ok(Column::Ints(ints_a.iter().zip(ints_b).map(|(a, b)| {a $op b}).collect())),
            _ => Err(stringify!($op, "not supported for non int/float ops").to_string())
        }
    }
}

pub fn add(a: Column, b: Column) -> Result<Column, String> {
    math_op_columns!(+, a, b)
}

pub fn multiply(a: Column, b: Column) -> Result<Column, String> {
    math_op_columns!(*, a, b)
}

pub fn subtract(a: Column, b: Column) -> Result<Column, String> {
    math_op_columns!(-, a, b)
}

pub fn divide(a: Column, b: Column) -> Result<Column, String> {
    math_op_columns!(/, a, b)
}

macro_rules! boolean_op_columns {
    ($op: tt, $left: expr, $right: expr) => {
        match ($left, $right) {
            (Column::Booleans(booleans_a), Column::Booleans(booleans_b)) => Ok(Column::Booleans(booleans_a.iter().zip(booleans_b).map(|(&a, b)| {a $op b}).collect())),
            _ => Err(stringify!($op, "only for supported booleans").to_string())
        }
    }
}
pub fn and(a: Column, b: Column) -> Result<Column, String> {
    boolean_op_columns!(&&, a, b)
}

pub fn or(a: Column, b: Column) -> Result<Column, String> {
    boolean_op_columns!(||, a, b)
}

macro_rules! comparison {

    ($op: tt, $left: expr, $right: expr) => {
        match ($left, $right) {
            (Column::Ints(a), Column::Ints(b)) => ok_op_bool!($op, a, b),
            (Column::Floats(a), Column::Floats(b)) => ok_op_bool!($op, a, b),
            (Column::Strings(a), Column::Strings(b)) => Ok(Column::Booleans(a.iter().zip(b).map(|(a, b)| { a.clone() $op b}).collect())),
            (Column::Dates(a), Column::Dates(b)) => ok_op_bool!($op, a, b),
            (Column::Floats(a), Column::Ints(b)) => Ok(Column::Booleans(a.iter().zip(b).map(|(&a, b)| { a $op (b as f64)}).collect())),
            (Column::Ints(a), Column::Floats(b)) => Ok(Column::Booleans(a.iter().zip(b).map(|(&a, b)| { (a as f64) $op b}).collect())),
            _ => Err("cannot compare ops".to_string())
        }
    }
}

pub fn equals(a: Column, b:Column) -> Result<Column, String> {
    comparison!(==, a, b)
}

pub fn not_equals(a: Column, b: Column) -> Result<Column, String> {
    comparison!(!=, a, b)
}

pub fn greater_than(a: Column, b: Column) -> Result<Column, String> {
    comparison!(>, a, b)
}

pub fn greater_than_equals(a: Column, b: Column) -> Result<Column, String> {
    comparison!(>=, a, b)
}

pub fn less_than(a: Column, b: Column) -> Result<Column, String> {
    comparison!(<, a, b)
}

pub fn less_than_equals(a: Column, b: Column) -> Result<Column, String> {
    comparison!(<=, a, b)
}
