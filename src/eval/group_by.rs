use crate::parser::ParserNode;
use crate::table::{Table, Column, NamedColumn};
use crate::result::{SqlResult, SqlError};
use crate::ops::OpContext;

use super::columns;

use std::collections::HashMap;
use crate::result::ErrorType::Runtime;
use std::hash::Hash;


pub (super) struct Grouped {
    groups: Vec<Table>,
}

pub (super) enum Either {
    Group(Grouped),
    Table(Table),
}

pub (super) fn eval(maybe_group_by_node: Option<ParserNode>,
                    table: Table,
                    op_context: &mut OpContext) -> SqlResult<Either> {

    match maybe_group_by_node {
        None => Ok(Either::Table(table)),

        Some(node) => {
            let (_, _, mut children) = node.release();

            let columns_node = children
                .pop_front()
                .ok_or(SqlError::new("group by needs items to group by", Runtime))?;

            let evaluated_keys = columns::eval(Some(columns_node), op_context, &table)?;

            let (assignments, key_tables) = keys_to_assignments(&evaluated_keys);

            Ok(Either::Group(Grouped {
                groups: key_tables.into_iter().enumerate().map(|(num, mut table)| {
                    let selector = assignments.iter().map(|assignment| {
                        assignment == &num
                    }).collect();

                    let selected_rows = table.where_(selector);

                    for column in selected_rows.into_columns() {
                        table.push(column, None)
                    }

                    table
                    }).collect()
                })
            )
        }
    }
}

fn keys_to_assignments(grouped_by_keys: &Table) -> (Vec<usize>, Vec<Table>) {
    let mut counter = 0;

    let mut hist: HashMap<u64, usize> = HashMap::new();

    let mut assignments = Vec::new();
    let mut representative_rows = Vec::new();

    for i in 0..grouped_by_keys.len() {
        let hash = grouped_by_keys.hash_row(i);

        if !hist.contains_key(&hash) {
            representative_rows.push(i);
            hist.insert(hash.clone(), counter);
            counter += 1;
        }

        assignments.push(hist[&hash]);
    }

    // grab one row per group
    let tables = representative_rows.into_iter().map(|i| {
        let mut selector = vec![false; grouped_by_keys.len()];
        selector[i] = true;

        grouped_by_keys.where_(selector)
    }).collect();

    (assignments, tables)

}