use crate::parser::ParserNode;
use crate::table::{Table, Column};
use crate::result::{SqlResult, SqlError};
use crate::ops::OpContext;

use super::columns;

use std::collections::HashMap;
use crate::result::ErrorType::Runtime;


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

            let group_by_keys = columns::eval(Some(columns_node), op_context, &table)?;

            // group_by_keys should either be the same length, or of length 1, a length of 1 is a degenerate case

            if group_by_keys.len() == 1 {

            }


        }
    }
}