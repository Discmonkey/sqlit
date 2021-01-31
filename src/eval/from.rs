use crate::parser::ParserNode;
use crate::table::{Store as TableContext, Table};
use crate::result::{SqlResult, SqlError};
use crate::ops::OpContext;
use crate::result::ErrorType::{Syntax};

fn from_statement_to_table<'store_life_time>(node: ParserNode, _ops: &mut OpContext, tables: &'store_life_time TableContext) -> SqlResult<&'store_life_time Table> {
    let (_, mut tokens, _) = node.release();

    tables.get(tokens.pop_front().unwrap().get_text())
}

pub (super) fn eval<'store_life_time>(root: ParserNode,
                          op_context: &mut OpContext,
                          table_context: &'store_life_time TableContext) -> SqlResult<&'store_life_time Table> {

    let (_, _, mut children)  = root.release();

    if children.is_empty() {
        return Err(SqlError::new("from does not reference any tables", Syntax));
    }

    // first node will be our from statement
    let table = from_statement_to_table(children.pop_front().unwrap(), op_context, table_context)?;

    Ok(table)
}