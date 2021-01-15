use crate::parser::ParserNode;
use crate::table::{Store as TableContext, Table};
use crate::result::{SqlResult, SqlError};
use crate::ops::OpContext;
use crate::result::ErrorType::{Syntax};

fn from_statement_to_table(node: ParserNode, _ops: &mut OpContext, tables: &mut TableContext) -> SqlResult<Table> {
    let (_, mut tokens, _) = node.release();

    tables.get(tokens.pop_front().unwrap().get_text()).map(|t| {
        t.clone()
    })
}

pub (super) fn eval(root: Option<ParserNode>,
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