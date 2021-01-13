use crate::table::{Store as TableContext};
use crate::parser::{ParserNode, ParserNodeType};
use crate::ops::OpContext;
use crate::result::{SqlResult, SqlError};
use crate::result::ErrorType::{Runtime};
use crate::eval::commands::{PrintableTableNames, PrintableTables};

mod split;
mod from;
mod select;
mod columns;
mod commands;

pub fn eval(root: ParserNode, op_context: &mut OpContext, table_context: &mut TableContext) -> SqlResult<Box<dyn std::fmt::Display>> {
    match root.get_type() {
        ParserNodeType::Query => select::eval(root, op_context, table_context).map(|t| Box::new(t) as Box<dyn std::fmt::Display>),
        ParserNodeType::TablesCommand => Ok(Box::new(PrintableTableNames::from(table_context)) as Box<dyn std::fmt::Display>),
        ParserNodeType::ColumnsCommand => Ok(Box::new(PrintableTables::from(table_context)) as Box<dyn std::fmt::Display>),
        _ => Err(SqlError::new("command not recognized, please use one of [<columns>, <tables>, <select...>]", Runtime))
    }
}