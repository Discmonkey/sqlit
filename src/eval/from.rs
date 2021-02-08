use crate::parser::{ParserNode, ParserNodeType};
use crate::table::{Store as TableContext, Table};
use crate::result::{SqlResult, SqlError};
use crate::ops::OpContext;
use crate::result::ErrorType::{Syntax, Runtime};
use std::rc;
use std::rc::Rc;
use super::select;
use std::collections::VecDeque;
use crate::eval::select::AliasMap;


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
                None => t.clone(),
                Some(alias) => t.with_new_alias(alias.to_string())
            }
        })
    }

}

fn join(left: Table, right: Table, expression: ParserNode, inner: bool) -> Table {
    // combine each row in the the table on the left, with the table on the right

    // evaluate expression

    // where true write left/right row pairs

    // if all false, write left, null ---- null

    unimplemented!()

}

pub (super) fn eval(root: ParserNode, ops: &OpContext, table_context: &TableContext) -> SqlResult<Table> {
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
            from_statement_to_table(node, ops, table_context)
        }).collect::<SqlResult<VecDeque<Table>>>()?;

    let first = tables.pop_front().ok_or(SqlError::new("select target not found", Runtime))?;

    Ok(tables.into_iter()
        .zip(join_condition_nodes.into_iter()).fold(first, |table, next| {
        return table
    }))
}
