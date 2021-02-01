use crate::parser::ParserNode;
use crate::table::{Store as TableContext, Table};
use crate::result::{SqlResult, SqlError};
use crate::ops::OpContext;
use crate::result::ErrorType::{Syntax, Runtime};
use std::rc;
use std::rc::Rc;
use super::select;


fn from_statement_to_table(node: ParserNode, ops: &OpContext, tables: &TableContext) -> SqlResult<Rc<Table>> {
    let (_, mut tokens, mut children) = node.release();

    let table_name = tokens.pop_front().ok_or(SqlError::new("join table needs identifier", Runtime))?;

    if !children.is_empty() {
        select::eval(children.pop_front().unwrap(), ops, tables).map(|t| Rc::new(t))
    } else {
        tables.get(table_name.get_text())
    }
}

pub (super) fn eval(root: ParserNode, ops: &OpContext,
                          table_context: &TableContext) -> SqlResult<Rc<Table>> {

    let (_, _, mut children)  = root.release();

    if children.is_empty() {
        return Err(SqlError::new("from does not reference any tables", Syntax));
    }

    let mut tables: Vec<Rc<Table>> = children
        .into_iter()
        .map(|node| {
        from_statement_to_table(node, ops, table_context)
    }).collect::<SqlResult<Vec<Rc<Table>>>>()?;

    Ok(tables.pop().unwrap())
}
