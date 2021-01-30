use crate::ops::MapOp;
use crate::result::{SqlResult, SqlError};
use crate::table::Column;
use crate::ops::binary_ops::prepare_binary_args;
use crate::result::ErrorType::{Runtime, Type};
use super::binary_ops::MapType;

// binary ops
pub (super) struct Or{}
pub (super) struct And{}
pub (super) struct Equal{}
pub (super) struct NotEqual{}
pub (super) struct Xor{}

// single ops
pub (super) struct Not{}

// compare ops
pub (super) struct Less{}
pub (super) struct Greater{}
pub (super) struct LessOrEqual{}
pub (super) struct GreaterOrEqual{}

macro_rules! binary_op_bool {

    ($target_struct:ident, $op:tt) => {
        impl MapOp for $target_struct {
            fn apply(&self, arguments: &Vec<Column>) -> SqlResult<Column> {
                let inputs = prepare_binary_args(arguments)?;

                match (inputs.left, inputs.right) {
                    bb!(l, r) => {
                        Ok(Column::Booleans(binary_iterate!(l, r, inputs.sizes, |(a, b)| {*a $op *b})))
                    },
                    _ => Err(SqlError::new("boolean op can only be performed on two boolean columns", Type)),
                }

            }
        }
    }
}

binary_op_bool!(Or, ||);
binary_op_bool!(And, &&);
binary_op_bool!(Xor, ^);


macro_rules! right_side {
    ($l:ident, $r:ident, $sizes: expr, $op: tt) => {
        Ok(Column::Booleans(binary_iterate!($l, $r, $sizes, |(a, b)| {a $op b})))
    }
}

macro_rules! binary_op_comp {
    ($target_struct: ident, $op: tt) => {
        impl MapOp for $target_struct {
            fn apply(&self, arguments: &Vec<Column>) -> SqlResult<Column> {
                let inputs = prepare_binary_args(arguments)?;

                match (inputs.left, inputs.right) {
                    bb!(l, r) => right_side!(l, r, inputs.sizes, $op),
                    ss!(l, r) => right_side!(l, r, inputs.sizes, $op),
                    ii!(l, r) => right_side!(l, r, inputs.sizes, $op),
                    dd!(l, r) => right_side!(l, r, inputs.sizes, $op),
                    ff!(l, r) => right_side!(l, r, inputs.sizes, $op),
                    if_!(l, r) => Ok(Column::Booleans(binary_iterate!(l, r, inputs.sizes, |(a, b)| {*a as f64 $op *b}))),
                    fi!(l, r) => Ok(Column::Booleans(binary_iterate!(l, r, inputs.sizes, |(a, b)| {*a $op *b as f64}))),
                    _ => Err(SqlError::new("incompatible types for boolean comparison", Type))
                }
            }
        }
    }
}

binary_op_comp!(Equal, ==);
binary_op_comp!(NotEqual, !=);

macro_rules! binary_op_comp_relative {
    ($target_struct: ident, $op: tt) => {
        impl MapOp for $target_struct {
            fn apply(&self, arguments: &Vec<Column>) -> SqlResult<Column> {
                let inputs = prepare_binary_args(arguments)?;

                match (inputs.left, inputs.right) {
                    dd!(l, r) => right_side!(l, r, inputs.sizes, $op),
                    ff!(l, r) => right_side!(l, r, inputs.sizes, $op),
                    ii!(l, r) => right_side!(l, r, inputs.sizes, $op),
                    if_!(l, r) => Ok(Column::Booleans(binary_iterate!(l, r, inputs.sizes, |(a, b)| {(*a as f64) $op *b}))),
                    fi!(l, r) => Ok(Column::Booleans(binary_iterate!(l, r, inputs.sizes, |(a, b)| {*a $op (*b as f64)}))),
                    _ => Err(SqlError::new("incompatible types for boolean comparison", Type))
                }
            }
        }
    }
}

binary_op_comp_relative!(Less, <);
binary_op_comp_relative!(Greater, >);
binary_op_comp_relative!(LessOrEqual, <=);
binary_op_comp_relative!(GreaterOrEqual, >=);


impl MapOp for Not {
    fn apply(&self, mut arguments: &Vec<Column>) -> SqlResult<Column> {
        arg_check!(1, arguments, "not", !=);

        match &arguments[0] {
            Column::Booleans(b) => Ok(Column::Booleans(b.into_iter().map(|maybe_bool| maybe_bool.map(|b| !b)).collect())),
            _ => Err(SqlError::new("not operator can only be applied to boolean column", Type))
        }
    }
}