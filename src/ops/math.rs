use crate::ops::MapOp;
use crate::result::{SqlResult, SqlError};
use crate::table::Column;
use crate::ops::binary_ops::{prepare_binary_args, MapType};
use crate::result::ErrorType::Type;
use std::cmp::max;

// binary ops
pub (super) struct Multiply{}
pub (super) struct Add{}
pub (super) struct Subtract{}
pub (super) struct Divide{}
pub (super) struct Power{}

// reduce ops
pub (super) struct Sum{}
pub (super) struct Max{}
pub (super) struct Min{}
pub (super) struct Mean{}

macro_rules! map_op_impl {
    ($target_struct:ident, $op:tt) => {
        impl MapOp for $target_struct {
            fn apply(&self, arguments: Vec<Column>) -> SqlResult<Column> {
                let input = prepare_binary_args(arguments)?;

                match (input.left, input.right) {
                    (Column::Ints(i1), Column::Ints(mut i2)) => {
                        Ok(Column::Ints(binary_iterate!(i1, i2, input.sizes, |(a, b)| {a $op b})))
                    },

                    (Column::Floats(f1), Column::Floats(f2)) => {
                        Ok(Column::Floats(binary_iterate!(f1, f2, input.sizes, |(a, b)| {a $op b})))
                    },

                    (Column::Floats(f1), Column::Ints(i2)) => {
                        Ok(Column::Floats(binary_iterate!(f1, i2, input.sizes, |(a, b)| {a $op b as f64})))
                    },

                    (Column::Ints(i1), Column::Floats(mut f2)) => {
                        Ok(Column::Floats(binary_iterate!(i1, f2, input.sizes, |(a, b)| {a as f64 $op b})))
                    }

                    _ => Err(SqlError::new("mismatched types in binary op", Type))
                }
            }
        }
    }
}

map_op_impl!(Multiply, *);
map_op_impl!(Add, +);
map_op_impl!(Subtract, -);
map_op_impl!(Divide, /);
