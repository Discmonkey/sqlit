use crate::parser::ParserNode;
use crate::result::{SqlResult, SqlError};
use crate::table::Table;
use crate::result::ErrorType::{Runtime, Type};
use std::rc::Rc;

pub (super) fn eval(node: ParserNode, mut table: Table) -> SqlResult<Table> {
        let (_, _, mut children) = node.release();
        let literal = children.pop_front().ok_or(SqlError::new("empty limit clause", Runtime))?;

        let string_val = literal
            .get_tokens()
            .front()
            .map(|t| t.get_text().clone())
            .ok_or(SqlError::new("missing tokens in literal", Runtime))?;

        let limit_by = string_val.parse::<usize>()
            .map_err(|_| {
                SqlError::new("could not parse limit, only integers supported", Type)
            })?;

        table.limit(limit_by);

        Ok(table)
}