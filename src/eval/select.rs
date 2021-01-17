use crate::eval::{split, where_, limit};
use crate::table::{Table, Store as TableContext};
use crate::parser::ParserNode;
use crate::ops::OpContext;
use crate::result::{SqlResult};
use crate::eval::from;
use super::columns;
use super::order_by;

pub (super) fn eval(root: ParserNode, op_context: &mut OpContext, table_context: &mut TableContext) -> SqlResult<Table> {
    let parts = split::split(root)?;

    let from_table = from::eval(parts.from, op_context, table_context)?;

    let filtered_from = where_::eval(parts.where_, from_table, op_context)?;

    let selected_table = columns::eval(parts.columns, op_context, &filtered_from)?;

    let ordered_table = order_by::eval(parts.order_by, selected_table)?;

    let limited_table = limit::eval(parts.limit, ordered_table)?;

    Ok(limited_table)
}