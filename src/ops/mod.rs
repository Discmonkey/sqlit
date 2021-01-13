#[macro_use]
mod binary_ops;

mod math;

use crate::result::{SqlResult, SqlError};
use std::collections::HashMap;
use crate::table::Column;
use crate::result::ErrorType::{Lookup, Syntax, Runtime};
use crate::ops::math::{Add, Multiply, Subtract, Divide, Max, Min, Mean, Sum};

pub trait MapOp {
    fn apply(&self, arguments: Vec<Column>) -> SqlResult<Column>;
}

pub trait ReduceOp {
    fn reduce(&self, argument: Column) -> SqlResult<Column>;
}

pub struct OpContext {
    applies: HashMap<String, Box<dyn MapOp>>,
    reducers: HashMap<String, Box<dyn ReduceOp>>,
}

impl OpContext {
    pub fn new() -> Self {
        let mut context = OpContext{
            applies: HashMap::new(),
            reducers: HashMap::new(),
        };

        context.set_apply("+", Box::new(Add{}));
        context.set_apply("*", Box::new(Multiply{}));
        context.set_apply("-", Box::new(Subtract{}));
        context.set_apply("/", Box::new(Divide{}));

        context.set_reduce("max", Box::new(Max{}));
        context.set_reduce("min", Box::new(Min{}));
        context.set_reduce("mean", Box::new(Mean{}));
        context.set_reduce("sum", Box::new(Sum{}));

        context
    }

    pub fn apply(&self, function: &str, arguments: Vec<Column>) -> SqlResult<Column> {
        self.applies.get(function).map(|f| {
            f.apply(arguments)
        }).ok_or(SqlError::new("no such op", Lookup))?
    }

    pub fn reduce(&self, function: &str, argument: Column) -> SqlResult<Column> {
        self.reducers.get(function).map(|r| {
            r.reduce(argument)
        }).ok_or(SqlError::new("no such reducer", Lookup))?
    }

    pub fn dispatch(&self, function: &str, mut arguments: Vec<Column>) -> SqlResult<Column> {
        if self.applies.contains_key(function) {
            self.apply(function, arguments)
        } else {
            self.reduce(function, arguments.pop().ok_or(
                SqlError::new("reducer called without arguments", Runtime))?)
        }
    }

    pub fn set_apply(&mut self, function: &str, op: Box<dyn MapOp>) {
        self.applies.insert(function.to_string(), op);
    }

    pub fn set_reduce(&mut self, function: &str, op: Box<dyn ReduceOp>) {
        self.reducers.insert(function.to_string(), op);
    }

}