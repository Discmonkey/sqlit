use crate::ops::OpContext;
use crate::parser::ParserNode;
use crate::result::{SqlResult, SqlError};
use crate::table::{Table};
use crate::result::ErrorType;


fn parse(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Table> {
    Err(SqlError::new("not implemented", ErrorType::Type))
}