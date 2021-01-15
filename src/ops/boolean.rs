use crate::ops::MapOp;
use crate::result::{SqlResult, SqlError};
use crate::table::Column;
use crate::ops::binary_ops::prepare_binary_args;
use crate::result::ErrorType::{Syntax, Runtime, Type};
use super::binary_ops::MapType;

// binary ops
pub (super) struct Or{}
pub (super) struct And{}
pub (super) struct Not{}
pub (super) struct Equal{}
pub (super) struct NotEqual{}
pub (super) struct Xor{}

// single ops


// reduce ops
pub (super) struct Any{}
pub (super) struct All{}

macro_rules! binary_op_bool {

    ($target_struct:ident, $op:tt) => {
        impl MapOp for $target_struct {
            fn apply(&self, arguments: Vec<Column>) -> SqlResult<Column> {
                let inputs = prepare_binary_args(arguments)?;

                match (inputs.left, inputs.right) {
                    bb!(l, r) => {
                        Ok(Column::Booleans(binary_iterate!(l, r, inputs.sizes, |(a, b)| {a $op b})))
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
            fn apply(&self, arguments: Vec<Column>) -> SqlResult<Column> {
                let inputs = prepare_binary_args(arguments)?;

                match (inputs.left, inputs.right) {
                    bb!(l, r) => right_side!(l, r, inputs.sizes, $op),
                    ss!(l, r) => right_side!(l, r, inputs.sizes, $op),
                    ii!(l, r) => right_side!(l, r, inputs.sizes, $op),
                    dd!(l, r) => right_side!(l, r, inputs.sizes, $op),
                    ff!(l, r) => right_side!(l, r, inputs.sizes, $op),
                    if_!(l, r) => Ok(Column::Booleans(binary_iterate!(l, r, inputs.sizes, |(a, b)| {a as f64 $op b}))),
                    fi!(l, r) => Ok(Column::Booleans(binary_iterate!(l, r, inputs.sizes, |(a, b)| {a $op b as f64}))),
                    _ => Err(SqlError::new("incompatible types for boolean comparison", Type))
                }
            }
        }
    }
}

binary_op_comp!(Equal, ==);
binary_op_comp!(NotEqual, !=);

impl MapOp for Not {
    fn apply(&self, mut arguments: Vec<Column>) -> SqlResult<Column> {
        if arguments.len() != 1 {
            return Err(SqlError::new("not operator takes exactly one column", Runtime));
        }

        let next = arguments.pop().unwrap();

        match next {
            Column::Booleans(b) => Ok(Column::Booleans(b.into_iter().map(|bool| !bool).collect())),
            _ => Err(SqlError::new("not operator can only be applied to boolean column", Type))
        }
    }
}