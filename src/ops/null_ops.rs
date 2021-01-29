use crate::ops::MapOp;
use crate::result::SqlResult;
use crate::table::Column;

pub (super) struct IsNull {

}

impl MapOp for IsNull {
    fn apply(&self, arguments: Vec<Column>) -> SqlResult<Column> {
        unimplemented!();
    }
}