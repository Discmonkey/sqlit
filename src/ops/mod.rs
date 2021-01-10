use crate::result::{SqlResult, SqlError};
use std::collections::HashMap;
use crate::table::Column;
use crate::result::ErrorType::Lookup;

pub mod math;

pub trait ApplyOp {
    fn apply(self, arguments: Vec<Column>) -> SqlResult<Column>;
    fn name() -> &str;
}

pub trait ReduceOp {
    fn reduce(self, ) -> SqlResult<Column>;
    fn name() -> &str;
}

pub struct OpContext {
    applies: HashMap<String, Box<dyn ApplyOp>>,
    reducers: HashMap<String, Box<dyn ReduceOp>>,
}

impl OpContext {
    pub fn new() -> Self {
        OpContext{
            applies: HashMap::new(),
            reducers: HashMap::new(),
        }
    }

    pub fn apply(&self, function: &str, arguments: Vec<Column>) -> SqlResult<Column> {
        self.applies.get(function).map(|f| {
            f.apply(arguments)
        }).ok_or(SqlError::new("no such op", Lookup))?
    }

    pub fn reduce(&self, function: &str, argument: Vec<Column>) -> SqlResult<Column> {
        self.reducers.get(function).map(|r| {
            r.reduce(argument)
        }).ok_or(SqlError::new("no such reducer", Lookup))?
    }

    pub fn set_apply(&mut self, op: Box<dyn ApplyOp>) {
        self.applies.insert(op.name().to_string(), op);
    }

    pub fn set_reduce(&mut self, op: Box<dyn ReduceOp>) {
        self.reducers.insert(op.name().to_string(), op);
    }

}