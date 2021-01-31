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

pub (super) fn eval(root: ParserNode, op_context: &mut OpContext,
                    table_context: &TableContext) -> SqlResult<Table> {

    let parts = split::split(root)?;

    // when possible we want to use references to a table that currently exists, ie avoid copying 300,000 columns * 10 rows on larger tables
    // on the other hand, when there is a need to mutate the underlying structure (such as on a where or a sort), we do want the freedom
    // to mutate or assign to something
    let mut permanent_table = Table::new();
    let mut table = &permanent_table;

    if let Some(node) = parts.from {
        table = from::eval(node, op_context, table_context)?;
    }

    if let Some(node) = parts.where_ {
        permanent_table = where_::eval(node, table, op_context, table_context)?;
        table = &permanent_table;
    }

    let selected_table = if let Some(group_by) = parts.group_by {
        let grouped = group_by::eval(group_by, table, op_context, table_context)?;
        let mut column_selections = Vec::new();

        for _ in 0..grouped.groups.len() {
            column_selections.push(parts.columns.clone());
        }

        let evaluated = grouped.groups.into_iter()
            .zip(column_selections.into_iter())
            .map(|(t, columns)| {

                let selected = columns::eval(columns, op_context, &t, table_context)?;

                if selected.len() > 1 {
                    Err(SqlError::new("length of group by result is greater than one, \
                    are you sure you used aggregate functions?", Runtime))
                } else {
                    Ok(selected)
                }

            }).collect::<SqlResult<Vec<Table>>>()?;

        let merged_table = Table::from_tables(evaluated)?;

        if let Some(order) = parts.order_by {
            order_by::eval(order, &merged_table)?
        } else {
            merged_table
        }

    } else {
        columns::eval(parts.columns, op_context,
            if let Some(order) = parts.order_by {
                permanent_table = order_by::eval(order, &table)?;
                &permanent_table
            } else {
                table
            }, table_context)?
    };

    let limited_table = limit::eval(parts.limit, selected_table)?;

    Ok(limited_table)
}