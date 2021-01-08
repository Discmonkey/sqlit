use crate::table::{Table, Store as TableContext};
use crate::parser::{ParserNode, ParserNodeType};
use crate::ops::OpContext;
use crate::result::{SqlResult, SqlError};
use crate::result::ErrorType::{Type, Runtime};
use crate::eval::commands::{PrintableTableNames, PrintableTables};

mod split;
mod from;
mod select;
mod columns;
mod commands;

pub fn eval(root: ParserNode, op_context: &mut OpContext, table_context: &mut TableContext) -> SqlResult<Box<dyn std::fmt::Display>> {
    match root.get_type() {
        ParserNodeType::Query => select::eval(root, op_context, table_context).map(|t| Box::new(t)),
        ParserNodeType::TablesCommand => Box::new(PrintableTableNames::from(table_context)),
        ParserNodeType::ColumnsCommand => Box::new(PrintableTables::from(table_context)),
        _ => Err(SqlError::new("command not recognized, please use one of [<columns>, <tables>, <select...>]", Runtime))
    }
}