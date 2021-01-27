use crate::ops::{ReduceOp};
use crate::result::SqlResult;
use crate::table::Column;

pub struct Count{}

impl ReduceOp for Count {
    fn reduce(&self, argument: Column) -> SqlResult<Column> {
        Ok(Column::Ints(vec![argument.len() as i64]))
    }
}