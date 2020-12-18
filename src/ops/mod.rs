use crate::result::{SqlResult, Evaluated};

pub mod math;

pub trait SqlOp {
    fn apply(args: Vec<SqlResult<Evaluated>>) -> SqlResult<Evaluated>;
}