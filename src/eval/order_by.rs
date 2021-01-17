use crate::table::{Table, Column};
use crate::parser::ParserNode;
use crate::result::{SqlResult, SqlError};
use crate::result::ErrorType::Runtime;
use std::collections::VecDeque;
use std::cmp::Ordering;

struct Order<'a> {
    pub column: &'a Column,
    pub asc: bool,
}

pub (super) fn eval(maybe_order_by: Option<ParserNode>, table: Table) -> SqlResult<Table>{
    match maybe_order_by {
        Some(mut order_by) => {
            let (_, _, clauses) = order_by.release();

            let orders = parse_into_order(clauses, &table)?;
            let mut sort_order: Vec<usize> = (0..table.len()).collect();

            sort_order.sort_by(|i, j| {
                for order in &orders {
                    let (mut a, mut b) = (i, j);
                    if order.asc {
                        a = j;
                        b = i;
                    }

                    let ordering = order.column.elem_order(*a, *b);

                    if ordering != Ordering::Equal {
                        return ordering;
                    }
                }

                Ordering::Equal
            });

            Ok(table)
        },

        None => Ok(table),
    }
}

fn parse_into_order(nodes: VecDeque<ParserNode>, table: &Table) -> SqlResult<Vec<Order>> {
    nodes.into_iter().map(|mut n| {
        let (_, mut tokens, _) = n.release();

        let column = tokens
            .pop_front()
            .map(|t| {
                table.column_search(t.get_text())
            })
            .ok_or(SqlError::new("need at least one column in order by", Runtime))??;

        let asc = tokens.pop_front().map(|t| {
            t.is("asc")
        }).unwrap_or(false);

        Ok(Order {
            column, asc
        })

    }).collect::<SqlResult<Vec<Order>>>()
}