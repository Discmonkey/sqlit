use crate::eval::{split, where_, limit};
use crate::table::{Table, Store as TableContext};
use crate::parser::ParserNode;
use crate::ops::OpContext;
use crate::result::{SqlResult, SqlError};
use crate::eval::from;
use super::columns;
use super::order_by;
use super::group_by;
use crate::result::ErrorType::Runtime;

pub (super) fn eval(root: ParserNode, op_context: &mut OpContext, table_context: &mut TableContext) -> SqlResult<Table> {
    let parts = split::split(root)?;

    let from_table = from::eval(parts.from, op_context, table_context)?;

    let filtered_from = where_::eval(parts.where_, from_table, op_context)?;

    let selected_table = match group_by::eval(parts.group_by, filtered_from, op_context)? {
        group_by::Either::Group(grouped) => {
            let mut column_selections = Vec::new();
            for _ in 0..grouped.groups.len() {
                column_selections.push(parts.columns.clone());
            }

            let evaluated = grouped.groups.into_iter()
                .zip(column_selections.into_iter())
                .map(|(t, columns)| {

                let selected = columns::eval(columns, op_context, &t)?;

                if selected.len() > 1 {
                    Err(SqlError::new("length of group by result is greater than one, \
                    are you sure you used aggregate functions?", Runtime))
                } else {
                    Ok(selected)
                }

            }).collect::<SqlResult<Vec<Table>>>()?;

            let merged_table = Table::from_tables(evaluated)?;

            order_by::eval(parts.order_by, merged_table)?
        }

        group_by::Either::Table(table) => columns::eval(parts.columns.clone(), op_context, &order_by::eval(parts.order_by, table)?)?
    };

    let limited_table = limit::eval(parts.limit, selected_table)?;

    Ok(limited_table)
}