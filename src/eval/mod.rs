use crate::table::{Store as TableContext};
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

pub fn eval(root: ParserNode, op_context: &mut OpContext, table_context: &mut TableContext) -> SqlResult<Box<dyn std::fmt::Display>> {
    match root.get_type() {
        ParserNodeType::Query => select::eval(root, op_context, table_context).map(|t| Box::new(t) as Box<dyn std::fmt::Display>),
        _ => Err(SqlError::new("command not recognized, please use one of [<select...>]", Runtime))
    }
}