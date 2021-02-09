use crate::parser::{ParserNode, ParserNodeType};
use crate::table::{Store as TableContext, Table, Column, Store, NamedColumn};
use crate::result::{SqlResult, SqlError};
use crate::ops::OpContext;
use crate::result::ErrorType::{Syntax, Runtime, Type};
use std::rc;
use std::rc::Rc;
use super::select;
use std::collections::VecDeque;
use crate::eval::select::AliasMap;
use crate::eval::columns::eval_expression;
use std::cmp::max;


fn from_statement_to_table(node: ParserNode,
                           ops: &OpContext,
                           tables: &TableContext) -> SqlResult<Table> {

    let (_, mut tokens, mut children) = node.release();

    if !children.is_empty() {
        let alias = tokens.pop_front().ok_or(SqlError::new("missing alias on select clause", Runtime))?;

        select::eval(children.pop_front().unwrap(), ops, tables).map(|t| {
            t.with_new_alias(alias.to_string())
        })
    } else {
        let table_name = tokens.pop_front().ok_or(SqlError::new("table name required", Runtime))?;
        let maybe_alias = tokens.pop_front();

        tables.get(table_name.get_text()).map(|t| {
            match maybe_alias {
                Some(alias) => t.with_new_alias(alias.to_string()),
                None => t.clone(),
            }
        })
    }

}

fn join(left: Table, right: Table, expression: ParserNode, inner: bool, ops: &OpContext, store: &Store) -> SqlResult<Table> {

    // if all false, write left, null --
    let mut destination_columns: Vec<(&str, Column)> = left.as_columns().into_iter().map(|(name, c) | {
        (name, c.clone())
    }).collect();

    right.as_columns().into_iter().for_each(|(name, c) | {
        destination_columns.push((name, c.clone()));
    });

    for row in 0..left.len() {
        let mut temp_table = left.row(row).unwrap();

        for col in right.to_columns().into_iter() {
            temp_table.push(col, Some(right.alias_ref()))
        }

        let evaluated = eval_expression(expression.clone(), ops, &temp_table, store)?;

        if let Column::Booleans(b) = evaluated.column.as_ref() {

            let selected = temp_table.where_(b);

            for _ in 0..max(1, selected.len()) {
                for (num, col) in temp_table.to_columns().into_iter().take(left.num_columns()).enumerate() {
                    destination_columns[num].1.extend(col.column.as_ref());
                }
            }

            if selected.len() == 0 {
                for num in left.num_columns()..temp_table.num_columns() {
                    destination_columns[num].1.push_null();
                }
            } else {
                for (num, col) in selected.to_columns().into_iter().enumerate().skip(left.num_columns()) {
                    destination_columns[num].1.extend(col.column.as_ref());
                }
            }
        } else {
            return Err(SqlError::new("join condition must evaluate to boolean", Type))
        }
    }

    let mut t = Table::new();

    destination_columns.into_iter().enumerate().for_each(|(num, (name, col))| {

        let table_name = if num < left.num_columns() {
            left.alias_ref()
        } else {
            right.alias_ref()
        };

        t.push(NamedColumn {
            column: Rc::new(col),
            name: name.to_string()
        }, Some(table_name));
    });

    Ok(t)
}

pub (super) fn eval(root: ParserNode, ops: &OpContext, store: &Store) -> SqlResult<Table> {
    let (_, _, mut children)  = root.release();

    if children.is_empty() {
        return Err(SqlError::new("from does not reference any tables", Syntax));
    }

    let (table_nodes, join_condition_nodes): (Vec<ParserNode>, Vec<ParserNode>) = children
        .into_iter()
        .partition(|parser_node| {
            parser_node.get_type() == &ParserNodeType::FromStatement
        });

    let mut tables: VecDeque<Table> = table_nodes
        .into_iter()
        .map(|node| {
            from_statement_to_table(node, ops, store)
        }).collect::<SqlResult<VecDeque<Table>>>()?;

    let first = tables.pop_front().ok_or(SqlError::new("select target not found", Runtime))?;

    let maybe_t = tables
        .into_iter()
        .zip(join_condition_nodes.into_iter()).try_fold(first, |current_join, (table_to_join, expression)| {
            join(current_join, table_to_join, expression, false, ops, store)
        })?;

    Ok(maybe_t)
}
