use crate::eval::split;
use crate::table::{Table, TableContext};
use crate::parser::ParserNode;
use crate::ops::OpContext;
use crate::result::SqlResult;


pub (super) fn eval(root: ParserNode, op_context: &mut OpContext, table_context: &mut TableContext) -> SqlResult<Table> {
    let mut parts = split::split(root)?;




}