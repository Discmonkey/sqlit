use crate::table::{Table, Column};
use crate::parser::ParserNode;
use crate::result::{SqlResult, SqlError};
use crate::result::ErrorType::Runtime;
use std::collections::VecDeque;
use std::cmp::Ordering;

struct Order<'a> {
    pub column: &'a Column,
    pub desc: bool,
}

pub (super) fn eval(order_by: ParserNode, table: &Table) -> SqlResult<Table>{
    let (_, _, clauses) = order_by.release();

    let orders = parse_into_order(clauses, &table)?;
    let mut sort_order: Vec<usize> = (0..table.len()).collect();

    sort_order.sort_by(|&i, &j| {
        for order in &orders {
            let (mut a, mut b) = (i, j);
            if order.desc {
                a = j;
                b = i;
            }

            let ordering = order.column.elem_order(a, b);

            if ordering != Ordering::Equal {
                return ordering;
            }
        }

        Ordering::Equal
    });

    Ok(table.order_by(sort_order))
}

fn parse_into_order(nodes: VecDeque<ParserNode>, table: &Table) -> SqlResult<Vec<Order>> {
    nodes.into_iter().map(|n| {
        let (_, mut tokens, _) = n.release();

        let column = tokens
            .pop_front()
            .map(|t| {
                table.column_search(t.get_text())
            })
            .ok_or(SqlError::new("need at least one column in order by", Runtime))??;

        let desc = tokens.pop_front().map(|t| {
            t.is("desc")
        }).unwrap_or(false);

        Ok(Order {
            column, desc
        })

    }).collect::<SqlResult<Vec<Order>>>()
}

#[cfg(test)]
mod test {
    use crate::ingest::{CsvFinder, SepFinder};
    use crate::tokenizer::Tokenizer;
    use crate::parser::rdp::RecursiveDescentParser;
    use crate::result::{SqlError, SqlResult};
    use crate::result::ErrorType::Runtime;
    use crate::{eval, ops, table};

    #[test]
    fn test_danceability() -> SqlResult<()>{

        let mut table_store = table::Store::from_paths(vec!["data/data.csv".to_string()], &(Box::new(CsvFinder{}) as Box<dyn SepFinder>), "null")
            .map_err(|_| SqlError::new("", Runtime))?;

        let t = Tokenizer::new();
        let tokens= t.tokenize("select name, danceability from data order by danceability".to_string());
        let parsed = RecursiveDescentParser::new(tokens).parse()?;
        let mut ops = ops::OpContext::new();

        let _ = eval::eval(parsed, &mut ops, &mut table_store)?;

        Ok(())
    }
}