use crate::ops::OpContext;
use crate::parser::{ParserNode, ParserNodeType};
use crate::result::{SqlResult, SqlError};
use crate::table::{Table, Column, NamedColumn};
use crate::result::ErrorType;
use crate::result::ErrorType::{Runtime, Syntax};
use std::collections::VecDeque;
use crate::tokenizer::Token;



pub (super) fn eval(mut node: Option<ParserNode>, op_context: &mut OpContext, table: &Table) -> SqlResult<Table> {
    let columns_root = node.ok_or(SqlError::new("no columns provided", Runtime))?;

    let (_, _, children) = columns_root.release();

    let mut columns = children.into_iter().map(|node| {
        eval_expression(node, op_context, table)
    }).collect::<SqlResult<VecDeque<NamedColumn>>>()?;

    let mut table = Table::new();

    while !columns.is_empty() {
        table.push(columns.pop_front().unwrap(), None);
    }

    Ok(table)
}

fn eval_expression(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<NamedColumn> {
    let (_, _, mut children) = node.release();

    let child = children.pop_front().ok_or(SqlError::new("empty expression", Runtime))?;

    eval_equality(child, op_context, table)
}

fn left_associative_helper(mut tokens: VecDeque<Token>,
                      mut nodes: VecDeque<ParserNode>,
                      op_context: &mut OpContext,
                      table: &Table,
                      next: fn (node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<NamedColumn>) -> SqlResult<NamedColumn> {

    let mut left_result = next(
        nodes
            .pop_front()
            .ok_or(SqlError::new("left operand missing", Syntax))?,
        op_context, table)?;

    while let Some(op) = tokens.pop_front() {
        let right_result = next(
            nodes
                .pop_front()
                .ok_or(SqlError::new("right operand missing", Syntax))?,
            op_context, table)?;

        left_result = NamedColumn {
            column: op_context.apply(op.get_text().as_str(), vec![
                left_result.column, right_result.column])?,
            name: op.to_string()
        }
    }

    Ok(left_result)
}

fn eval_equality(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<NamedColumn> {
    let (_, mut tokens, mut nodes) = node.release();
    left_associative_helper(tokens, nodes, op_context, table, eval_comparison)
}

fn eval_comparison(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<NamedColumn> {
    let (_, mut tokens, mut nodes) = node.release();
    left_associative_helper(tokens, nodes, op_context, table, eval_term)
}

fn eval_term(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<NamedColumn> {
    let (_, mut tokens, mut nodes) = node.release();
    left_associative_helper(tokens, nodes, op_context, table, eval_factor)
}

fn eval_factor(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<NamedColumn> {
    let (_, mut tokens, mut nodes) = node.release();
    left_associative_helper(tokens, nodes, op_context, table, eval_unary)
}

fn eval_unary(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<NamedColumn> {
    let (_, mut tokens, mut nodes) = node.release();

    let next_node = nodes.pop_front().ok_or(SqlError::new("expected value", Syntax))?;

    match tokens.pop_front() {
        Some(t) => {
            let evaluated = eval_unary(next_node, op_context, table)?;
            let op = t.get_text();

            Ok(NamedColumn {
                column: op_context.apply(op.as_str(), vec![evaluated.column])?,
                name: op.clone(),
            })
        },

        None => eval_primary(next_node, op_context, table)
    }
}

fn eval_primary(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<NamedColumn> {
    let (_, _, mut nodes) = node.release();

    let next_node = nodes.pop_front().ok_or(SqlError::new("expected value", Syntax))?;

    match next_node.get_type() {
        ParserNodeType::Identifier => eval_identifier(next_node, table),
        _ => Err(SqlError::new("only identifiers currently supported", Runtime))
    }
}

fn eval_identifier(node: ParserNode, table: &Table) -> SqlResult<NamedColumn> {

    let (_, mut tokens, _) = node.release();

    match tokens.len() {
        1 => {
            let column_identifier = tokens.pop_front().unwrap();
            table.column_search(column_identifier.get_text().as_str()).map(|c| NamedColumn {
                column: c.clone(),
                name: column_identifier.get_text().clone(),
            })
        },
        2 => {
            let table_identifier = tokens.pop_front().unwrap();
            let column_identifier = tokens.pop_front().unwrap();

            table.column(
                table_identifier.get_text().as_str(),
                column_identifier.get_text().as_str(),
            ).map(|c| {
                NamedColumn {
                    column: c.clone(),
                    name: column_identifier.get_text().clone()
                }
            }).ok_or(SqlError::new("column not found", Runtime))
        },
        _ => Err(SqlError::new("identifier can have either 1 or 2 related tokens a | a.b", Runtime))
    }
}