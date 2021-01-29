use crate::ops::MapOp;
use crate::result::{SqlResult, SqlError};
use crate::table::Column;

pub (super) struct IsNull {

}

pub (super) struct NotNull {

}

impl MapOp for IsNull {
    fn apply(&self, mut arguments: Vec<Column>) -> SqlResult<Column> {
        arg_check!(1, arguments, "IsNull");

        unimplemented!()


    }
}

impl MapOp for NotNull {
    fn apply(&self, arguments: Vec<Column>) -> SqlResult<Column> {
        unimplemented!()
    }
}