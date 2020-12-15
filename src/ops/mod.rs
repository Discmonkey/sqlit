use crate::result::SqlResult;

pub mod math;

pub trait SqlOp {
    fn apply(args: Vec<SqlResult>) -> SqlResult;
}