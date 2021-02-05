use crate::ops::{MapOp, ReduceOp};
use crate::result::{SqlResult, SqlError};
use crate::table::Column;
use crate::ops::binary_ops::{prepare_binary_args, MapType};
use crate::result::ErrorType::{Type, Runtime};

// binary ops
pub (super) struct Multiply{}
pub (super) struct Add{}
pub (super) struct Subtract{}
pub (super) struct Divide{}
pub (super) struct Mod{}

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
            fn apply(&self, arguments: Vec<&Column>) -> SqlResult<Column> {
                let input = prepare_binary_args(arguments)?;

                match (input.left, input.right) {
                    ii!(i1, i2) => {
                        Ok(Column::Ints(binary_iterate!(i1, i2, input.sizes, |(a, b)| {a $op b})))
                    },

                    ff!(f1, f2) => {
                        Ok(Column::Floats(binary_iterate!(f1, f2, input.sizes, |(a, b)| {a $op b})))
                    },

                    fi!(f1, i2) => {
                        Ok(Column::Floats(binary_iterate!(f1, i2, input.sizes, |(a, b)| {a $op *b as f64})))
                    },

                    if_!(i1, f2) => {
                        Ok(Column::Floats(binary_iterate!(i1, f2, input.sizes, |(a, b)| {*a as f64 $op b})))
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
map_op_impl!(Mod, %);

macro_rules! vec_wrap {
    ($expr: expr) => {
        vec![Some($expr)]
    }
}
macro_rules! numeric_reduce_impl {
    ($target_struct:ident, $op_int:tt, $op_float:tt) => {
        impl ReduceOp for $target_struct {
            fn reduce(&self, argument: &Column) -> SqlResult<Column> {

                if argument.len() == 0 {
                    Err(SqlError::new("cannot reduce an empty column", Runtime))
                } else {
                    match argument {
                        Column::Ints(i) => Ok(Column::Ints(vec_wrap!(i.into_iter().filter_map(|v| *v).$op_int().unwrap()))),
                        Column::Floats(f) => Ok(Column::Floats(vec_wrap!(f.into_iter().filter_map(|v| *v).$op_float()))),
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
    fn reduce(&self, argument: &Column) -> SqlResult<Column> {
        match argument {
            Column::Floats(f) => {
                Ok(Column::Floats(
                    vec![
                        Some(f.iter().map(|float| float.unwrap_or(0.0)).sum())]))
            },
            Column::Ints(i) => {
                Ok(Column::Ints(
                    vec![
                        Some(i.iter().map(|int| int.unwrap_or(0)).sum())]))
            },
            _ => Err(SqlError::new("cannot sum non-numeric type", Type))
        }
    }
}

impl ReduceOp for Mean {
    fn reduce(&self, argument: &Column) -> SqlResult<Column> {
        match argument {
            Column::Floats(f) => {
                Ok(Column::Floats(
                    vec_wrap!(f.into_iter().filter_map(|v| v.as_ref()).enumerate().fold(0.0, |rolling_mean, (counter, next)| {
                            let c = counter as f64;

                            if counter > 0 {
                                rolling_mean * (c - 1.0) / c + *next / c
                            } else {
                                *next
                            }
                        }))
                    ))
            }

            Column::Ints(i) => Ok(Column::Floats(
                    vec_wrap!(i.into_iter()
                            .filter_map(|f| {
                                f.as_ref()
                            })
                            .enumerate().fold(0.0, |rolling_mean, (counter, next)| {
                            let c = counter as f64;

                            if counter > 0 {
                                rolling_mean * (c - 1.0) / c + *next as f64 / c
                            } else {
                                *next as f64
                            }
                        }))
                    )),

            _ => Err(SqlError::new("cannot take mean of non-numeric column", Type))
        }
    }
}