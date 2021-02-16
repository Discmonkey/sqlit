use crate::table::{Store, Table};
use crate::parser::{ParserNode, ParserNodeType};
use crate::ops::OpContext;
use crate::result::{SqlResult, SqlError};
use crate::result::ErrorType::{Runtime};

mod split;
mod from;
mod select;
mod columns;
mod where_;
mod limit;
mod order_by;
mod group_by;
mod into;

pub fn eval(root: ParserNode, op_context: &mut OpContext, table_context: &Store) -> SqlResult<Table> {
    match root.get_type() {
        ParserNodeType::Query => select::eval(root, op_context, table_context),
        _ => Err(SqlError::new("command not recognized, please use one of [<select...>]", Runtime))
    }
}