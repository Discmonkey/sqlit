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
                           tables: &TableContext,
                           a_map: &mut AliasMap) -> SqlResult<Rc<Table>> {

    let (_, mut tokens, mut children) = node.release();

    let table_name = tokens.pop_front().ok_or(SqlError::new("join table needs identifier", Runtime))?;

    let result = if !children.is_empty() {
        select::eval(children.pop_front().unwrap(), ops, tables).map(|t| Rc::new(t))
    } else {
        tables.get(table_name.get_text())
    };

    result.iter().for_each(|r| {
        a_map.insert(table_name.to_string(), r.alias().to_string());
    });

    result
}

fn join(left: Rc<Table>, right: Rc<Table>, expression: ParserNode) {

}

pub (super) fn eval(root: ParserNode, ops: &OpContext,
                          table_context: &TableContext, a_map: &mut AliasMap) -> SqlResult<Rc<Table>> {


    let (_, _, mut children)  = root.release();

    if children.is_empty() {
        return Err(SqlError::new("from does not reference any tables", Syntax));
    }

    let (table_nodes, join_condition_nodes): (Vec<ParserNode>, Vec<ParserNode>) = children
        .into_iter()
        .partition(|parser_node| {
            parser_node.get_type() == &ParserNodeType::FromStatement
        });

    let mut tables: VecDeque<Rc<Table>> = table_nodes
        .into_iter()
        .map(|node| {
            from_statement_to_table(node, ops, table_context, a_map)
        }).collect::<SqlResult<VecDeque<Rc<Table>>>>()?;

    let first = tables.pop_front().ok_or(SqlError::new("select target not found", Runtime))?;

    Ok(tables.into_iter()
        .zip(join_condition_nodes.into_iter()).fold(first, |table, next| {
        return table
    }))
}
