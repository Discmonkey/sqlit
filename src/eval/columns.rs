use crate::ops::OpContext;
use crate::parser::ParserNode;
use crate::result::{SqlResult, SqlError};
use crate::table::{Table, Column};
use crate::result::ErrorType;
use crate::result::ErrorType::{Runtime, Syntax};
use std::collections::VecDeque;
use crate::tokenizer::Token;

struct NamedColumn {
    pub name: String
}

pub (super) fn eval(mut node: Option<ParserNode>, op_context: &mut OpContext, table: &Table) -> SqlResult<Table> {

    let columns_root = node.ok_or(SqlError::new("no columns provided", Runtime))?;

    let (_, _, children) = columns_root.release();

    let columns  = children.into_iter().map(|node| {
        eval_expression(node, op_context, table)
    }).collect::<SqlResult<Vec<Column>>>()?;



    unimplemented!()
}

fn eval_expression(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Column> {
    let (_, _, mut children) = node.release();

    let child = children.pop_front().ok_or(SqlError::new("empty expression", Runtime))?;

    eval_equality(child, op_context, table)
}

fn left_associative_helper(mut tokens: VecDeque<Token>,
                      mut nodes: VecDeque<ParserNode>,
                      op_context: &mut OpContext,
                      table: &Table,
                      next: fn (node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Column>) -> SqlResult<Column> {

    let mut left_node = next(
        nodes
            .pop_front()
            .ok_or(SqlError::new("left operand missing", Syntax))?,
        op_context, table)?;

    while let Some(op) = tokens.pop_front() {
        let right_node = next(
            nodes
                .pop_front()
                .ok_or(SqlError::new("right operand missing", Syntax))?,
            op_context, table)?;

        left_node = op_context.apply(op.get_text().as_str(), vec![left_node, right_node])?;
    }

    Ok(left_node)
}

fn eval_equality(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Column> {
    let (_, mut tokens, mut nodes) = node.release();
    left_associative_helper(tokens, nodes, op_context, table, eval_comparison)
}

fn eval_comparison(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Column> {
    let (_, mut tokens, mut nodes) = node.release();
    left_associative_helper(tokens, nodes, op_context, table, eval_term)
}

fn eval_term(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Column> {
    let (_, mut tokens, mut nodes) = node.release();
    left_associative_helper(tokens, nodes, op_context, table, eval_factor)
}

fn eval_factor(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Column> {
    let (_, mut tokens, mut nodes) = node.release();
    left_associative_helper(tokens, nodes, op_context, table, eval_unary)
}

fn eval_unary(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Column> {
    let (_, mut tokens, mut nodes) = node.release();

    let next_node = nodes.pop_front().ok_or(SqlError::new("expected value", Syntax))?;

    match tokens.pop_front() {
        Some(t) => {
            let evaluated = eval_unary(next_node, op_context, table)?;
            op_context.apply(t.get_text().as_str(), vec![evaluated])
        },

        None => eval_primary(next_node, op_context, table)
    }
}

fn eval_primary(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<Column> {
    unimplemented!()
}

