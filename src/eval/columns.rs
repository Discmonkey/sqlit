use crate::ops::OpContext;
use crate::parser::{ParserNode, ParserNodeType};
use crate::result::{SqlResult, SqlError};
use crate::table::{Table, NamedColumn};
use crate::result::ErrorType::{Runtime, Syntax};
use std::collections::VecDeque;
use crate::tokenizer::{Token, TokenType};
use crate::build_column::build_column;
use crate::parser::ParserNodeType::{StarOperator, Identifier};
use crate::parser::rdp::RecursiveDescentParser;


pub (super) fn eval(node: Option<ParserNode>, op_context: &mut OpContext, table: &Table) -> SqlResult<Table> {
    let columns_root = node.ok_or(SqlError::new("no columns provided", Runtime))?;

    let (_, _, mut children) = columns_root.release();

    children = expand_star_operator(children, table)?;

    let mut columns = children.into_iter().map(|node| {
        eval_expression(node, op_context, table)
    }).collect::<SqlResult<VecDeque<NamedColumn>>>()?;

    let mut table = Table::new();

    while !columns.is_empty() {
        table.push(columns.pop_front().unwrap(), None);
    }

    Ok(table)
}

fn expand_star_operator(nodes: VecDeque<ParserNode>, table: &Table) -> SqlResult<VecDeque<ParserNode>> {
    let mut expanded_nodes = VecDeque::new();

    for n in nodes.into_iter() {
        if n.get_type() == &StarOperator {

            for (name, _) in table.meta().columns.into_iter() {
                let mut tokens = VecDeque::new();
                tokens.push_back(Token::new(name, TokenType::Identifier));

                expanded_nodes.push_back(RecursiveDescentParser::new(tokens).parse_expression()?);
            }

        } else {
            expanded_nodes.push_back(n);
        }
    }

    Ok(expanded_nodes)
}

pub (super) fn eval_expression(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<NamedColumn> {
    let (_, _, mut children) = node.release();

    let child = children.pop_front().ok_or(SqlError::new("empty expression", Runtime))?;

    let mut named_column = eval_equality(child, op_context, table)?;

    if let Some(node) = children.pop_front() {
        named_column.name = node.get_tokens().front().map(|t| {
            t.get_text().clone()
        }).ok_or(SqlError::new("as must be followed by identifier", Runtime))?;
    }

    Ok(named_column)
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
            column: op_context.apply(op.get_text().as_str(), &vec![
                left_result.column, right_result.column])?,
            name: op.to_string()
        }
    }

    Ok(left_result)
}

fn eval_equality(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<NamedColumn> {
    let (_, tokens, nodes) = node.release();
    left_associative_helper(tokens, nodes, op_context, table, eval_comparison)
}

fn eval_comparison(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<NamedColumn> {
    let (_, tokens, nodes) = node.release();
    left_associative_helper(tokens, nodes, op_context, table, eval_term)
}

fn eval_term(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<NamedColumn> {
    let (_, tokens, nodes) = node.release();
    left_associative_helper(tokens, nodes, op_context, table, eval_factor)
}

fn eval_factor(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<NamedColumn> {
    let (_, tokens, nodes) = node.release();
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
                column: op_context.apply(op.as_str(), &vec![evaluated.column])?,
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
        ParserNodeType::Function => eval_function(next_node, op_context, table),
        ParserNodeType::Literal => eval_literal(next_node),
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

fn eval_function(node: ParserNode, op_context: &mut OpContext, table: &Table) -> SqlResult<NamedColumn> {
    let(_, mut tokens, mut nodes) = node.release();

    let table = eval(nodes.pop_front(), op_context, table)?;
    let columns = table.into_columns().into_iter().map(|c| c.column).collect();

    let op = tokens.pop_front().ok_or(SqlError::new("function without name", Syntax))?;

    op_context.dispatch(op.get_text().as_str(), &columns).map(|col| {
        NamedColumn {
            column: col,
            name: op.get_text().clone()
        }
    })
}

fn eval_literal(node: ParserNode) -> SqlResult<NamedColumn>{
    let (_, tokens, _) = node.release();

    Ok(NamedColumn {
        column: build_column(tokens.iter().map(|t| t.get_text().clone()).collect(), "nan"),
        name: "".to_string(),
    })
}

