use crate::ops::OpContext;
use crate::parser::ParserNode;
use crate::result::{SqlResult, SqlError};
use crate::table::{Table, Column};
use crate::result::ErrorType;
use crate::result::ErrorType::Runtime;

struct NamedColumn {
    pub name: String
}

pub (super) fn eval(mut node: Option<ParserNode>, op_context: &mut OpContext, table: &Table) -> SqlResult<Table> {

    let columns_root = node.ok_or(SqlError::new("no columns provided", Runtime))?;

    let (_, _, children) = columns_root.release();

    let columns: SqlResult<Vec<Column>> = children.into_iter().map(|node| {
        eval_expression(node, op_context, table)
    }).collect()?;



    unimplemented!()
}

fn eval_expression(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Column> {
    let (_, _, mut children) = node.release();

    let child = children.pop_front().ok_or(SqlError::new("empty expression", Runtime))?;

    eval_equality(child, op_context, table)
}

fn eval_function_helper()

fn eval_equality(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Column> {
    unimplemented!()
}

fn eval_comparison(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Column> {
    unimplemented!()
}

fn eval_term(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Column> {
    unimplemented!()
}

fn eval_factor(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Column> {
    unimplemented!()
}

fn eval_unary(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Column> {
    unimplemented!()
}

fn eval_primary(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Column> {
    unimplemented!()
}

