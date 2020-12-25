use crate::result::{SqlResult, Evaluated};
use std::collections::HashMap;

pub mod math;

pub trait SqlOp {
    fn apply(self, args: Vec<SqlResult<Evaluated>>) -> SqlResult<Evaluated>;
}

pub type OpContext = HashMap<String, Box<dyn SqlOp>>;

pub fn new_op_context() -> OpContext {
    // TODO fill this out with, you know, ops
    return HashMap::new();
}