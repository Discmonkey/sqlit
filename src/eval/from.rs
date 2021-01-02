use crate::parser::{ParserNode, ParserNodeType};
use crate::table::{TableContext, Table};
use crate::result::{SqlResult, SqlError};
use crate::ops::OpContext;
use crate::result::ErrorType::{Syntax, Type, Lookup};



fn from_statement_to_table(node: ParserNode, ops: &mut OpContext, tables: &mut TableContext) -> SqlResult<Table> {
    match tables.get(node.get_tokens().pop_front().unwrap().get_text()) {
        None => Err(SqlError::new("requested table does not exist", Lookup)),
        Some(table) => Ok(table.clone()), 
    }
}

pub (super) fn parse_from(root: Option<ParserNode>,
                          op_context: &mut OpContext,
                          table_context: &mut TableContext) -> SqlResult<Table> {

    if root.is_none() {
        return Ok(Table::new());
    }

    let (_, _, mut children)  = root.unwrap().release();

    if children.is_empty() {
        return Err(SqlError::new("from does not reference any tables", Syntax));
    }

    // first node will be our from statement
    let table = from_statement_to_table(children.pop_front().unwrap(), op_context, table_context)?;

    Ok(table)
}