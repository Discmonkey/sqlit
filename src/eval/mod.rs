use crate::table::{Table, TableContext};
use crate::parser::ParserNode;
use crate::ops::OpContext;

pub fn eval(root: ParserNode, op_context: &mut OpContext, table_context: &mut TableContext, is_sub_select: bool) -> Table {
    // break the root ast into its constituent parts

    // FROM: (table_context, ast) -> Table

    // WHERE: (Table, ast) -> Table

    // GROUP BY: (Table, ast) ->

    // SELECT: (Table, ast) ->
}