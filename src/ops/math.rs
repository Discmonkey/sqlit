use crate::ops::{MapOp, ReduceOp};
use crate::result::{SqlResult, SqlError};
use crate::table::Column;
use crate::ops::binary_ops::{prepare_binary_args, MapType};
use crate::result::ErrorType::{Type, Runtime};
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

trait FloatIterExt {
    fn float_min(&mut self) -> f64;
    fn float_max(&mut self) -> f64;
}

impl<T> FloatIterExt for T where T: Iterator<Item=f64> {
    fn float_min(&mut self) -> f64 {
        self.fold(f64::NAN, f64::min)
    }

    fn float_max(&mut self) -> f64 {
        self.fold(f64::NAN, f64::max)
    }
}

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

macro_rules! numeric_reduce_impl {
    ($target_struct:ident, $op_int:tt, $op_float:tt) => {
        impl ReduceOp for $target_struct {
            fn reduce(&self, argument: Column) -> SqlResult<Column> {

                if argument.len() == 0 {
                    Err(SqlError::new("cannot reduce an empty column", Runtime))
                } else {
                    match argument {
                        Column::Ints(i) => Ok(Column::Ints(vec![i.into_iter().$op_int().unwrap()])),
                        Column::Floats(f) => Ok(Column::Floats(vec![f.into_iter().$op_float()])),
                        _ => Err(SqlError::new("max function reduces only over ints and floats", Type))
                    }
                }
            }
        }
    }
}

numeric_reduce_impl!(Max, max, float_max);
numeric_reduce_impl!(Min, min, float_min);

impl ReduceOp for Sum {
    fn reduce(&self, argument: Column) -> SqlResult<Column> {
        match argument {
            Column::Floats(f) => Ok(Column::Floats(vec![f.into_iter().sum()])),
            Column::Ints(i) => Ok(Column::Ints(vec![i.into_iter().sum()])),
            _ => Err(SqlError::new("cannot sum non-numeric type", Type))
        }
    }
}

impl ReduceOp for Mean {
    fn reduce(&self, argument: Column) -> SqlResult<Column> {
        match argument {
            Column::Floats(f) => {
                Ok(Column::Floats(
                    vec![
                        f.into_iter().enumerate().fold(0.0, |rolling_mean, (counter, next)| {
                            let c = counter as f64;

                            if counter > 0 {
                                rolling_mean * (c - 1.0) / c + next / c
                            } else {
                                next
                            }
                        })]))
            }

            Column::Ints(i) => Ok(Column::Floats(
                    vec![
                        i.into_iter().enumerate().fold(0.0, |rolling_mean, (counter, next)| {
                            let c = counter as f64;

                            if counter > 0 {
                                rolling_mean * (c - 1.0) / c + next as f64 / c
                            } else {
                                next as f64
                            }
                        })])),

            _ => Err(SqlError::new("cannot take mean of non-numeric column", Type))
        }
    }
}