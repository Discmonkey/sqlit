use crate::eval::{split, where_, limit};
use crate::table::{Table, Store as TableContext};
use crate::parser::ParserNode;
use crate::ops::OpContext;
use crate::result::{SqlResult, SqlError};
use crate::eval::from;
use super::columns;
use super::order_by;
use super::group_by;
use super::into;
use crate::result::ErrorType::Runtime;
use std::rc::Rc;
use std::collections::HashMap;

pub (super) type AliasMap = HashMap<String, String>;

pub (super) fn eval(root: ParserNode, op_context: &OpContext,
                    table_context: &TableContext) -> SqlResult<Table> {

    let parts = split::split(root)?;

    let mut table = match parts.from {
        Some(node) => from::eval(node, op_context, table_context)?,
        None => Table::new(),
    };

    if let Some(node) = parts.where_ {
        table = where_::eval(node, &table, op_context, table_context)?;
    }

    let selected_table = if let Some(group_by) = parts.group_by {
        let grouped = group_by::eval(group_by, &table, op_context, table_context)?;
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
        let mut temp;
        columns::eval(parts.columns, op_context,
            if let Some(order) = parts.order_by {
                temp = order_by::eval(order, &table)?;

                &temp
            } else {
                &table
            }, table_context)?
    };

    let final_table = if let Some(limit) = parts.limit {
        limit::eval(limit, selected_table)?
    } else {
        selected_table
    };


    if let Some(into_node) = parts.into {
        into::eval(into_node, &final_table)?;

        Ok(Table::new())
    } else {
        Ok(final_table)
    }

}