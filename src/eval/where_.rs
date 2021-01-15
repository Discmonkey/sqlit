use crate::parser::ParserNode;
use crate::ops::OpContext;
use crate::table::{Store, Table, Column};
use crate::result::{SqlResult, SqlError};
use crate::result::ErrorType::{Runtime, Type};
use crate::eval::columns::eval_expression;
use crate::table::ColumnType::Boolean;

pub (super) fn eval(maybe_node: Option<ParserNode>, mut table: Table,
                    mut op_context: &mut OpContext) -> SqlResult<Table> {
    match maybe_node {
        None => Ok(table),
        Some(node) => {
            let (_, _, mut children) = node.release();
            let where_expression = children.pop_front().ok_or(SqlError::new("empty where clause", Runtime))?;

            let booleans = eval_expression(where_expression, &mut op_context, &table)?.column;

            match booleans {
                Column::Booleans(b) => {
                    table.where_(b);
                    Ok(table)
                }

                _ => Err(SqlError::new("where clause must evaluate to a boolean column", Type))
            }
        }
    }
}