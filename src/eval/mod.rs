use crate::table::{Table, Context as TableContext};
use crate::parser::{ParserNode, ParserNodeType};
use crate::ops::OpContext;
use crate::result::{SqlResult, SqlError};
use crate::result::ErrorType::Type;

mod split;
mod from;
mod select;
mod columns;

pub fn eval(root: ParserNode, op_context: &mut OpContext, table_context: &mut TableContext) -> SqlResult<Table> {
    match root.get_type() {
        ParserNodeType::Query => select::eval(root, op_context, table_context),
        _ => Err(SqlError::new("not implemented", Type))
    }
}