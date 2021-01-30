use crate::parser::ParserNode;
use crate::ops::OpContext;
use crate::table::{Table, Column};
use crate::result::{SqlResult, SqlError};
use crate::result::ErrorType::{Runtime, Type};
use crate::eval::columns::eval_expression;

pub (super) fn eval(node: ParserNode, table: &Table,
                    mut op_context: &mut OpContext) -> SqlResult<Table> {

    let (_, _, mut children) = node.release();
    let where_expression = children.pop_front().ok_or(SqlError::new("empty where clause", Runtime))?;

    let booleans = eval_expression(where_expression, &mut op_context, &table)?.column;

    match booleans {
        Column::Booleans(b) => {
            Ok(table.where_(b))
        }

        _ => Err(SqlError::new("where clause must evaluate to a boolean column", Type))
    }
}