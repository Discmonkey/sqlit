use crate::eval::split;
use crate::table::{Table, Store as TableContext};
use crate::parser::ParserNode;
use crate::ops::OpContext;
use crate::result::{SqlResult, SqlError};
use crate::result::ErrorType::Lookup;
use crate::eval::from;
use super::columns;


pub (super) fn eval(root: ParserNode, op_context: &mut OpContext, table_context: &mut TableContext) -> SqlResult<Table> {
    let mut parts = split::split(root)?;

    let from_table = from::eval(parts.from, op_context, table_context)?;

    let mut selected_table = columns::eval(parts.columns, op_context, &from_table)?;

    selected_table.limit(10);

    Ok(selected_table)
}