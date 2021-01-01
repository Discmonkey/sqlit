use crate::table::{Table, TableContext};
use crate::parser::ParserNode;
use crate::ops::OpContext;
use crate::result::{SqlResult, SqlError};
use crate::result::ErrorType::Type;

mod split;
mod from;


pub fn eval(root: ParserNode, op_context: &mut OpContext, table_context: &mut TableContext, is_sub_select: bool) -> SqlResult<Table> {
    // break the root ast into its constituent parts

    let mut parts = split::split(root)?;

    let mut maybe_from: Option<Table> = None;

    // if let Some(from) = parts.from {
    //     maybe_from = Some()
    // }

    // FROM: (table_context, ast) -> Table

    // WHERE: (Table, ast) -> Table

    // GROUP BY: (Table, ast) ->

    // SELECT: (Table, ast) ->

    Err(SqlError::new("not implemented", Type))
}