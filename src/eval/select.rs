use crate::eval::split;
use crate::table::{Table, Store as TableContext};
use crate::parser::ParserNode;
use crate::ops::OpContext;
use crate::result::{SqlResult, SqlError};
use crate::result::ErrorType::Lookup;
use crate::eval::from::parse_from;
use crate::column;
use crate::parser::ParserNodeType::Columns;
use crate::result::Evaluated::Column;


pub (super) fn eval(root: ParserNode, op_context: &mut OpContext, table_context: &mut TableContext) -> SqlResult<Table> {
    let mut parts = split::split(root)?;

    let table = parse_from(parts.from, op_context, table_context)?;

    if let Some(columns) = parts.columns {
        Ok(table)
    } else {
        Err(SqlError::new("columns are unspecified", Lookup))
    }




}