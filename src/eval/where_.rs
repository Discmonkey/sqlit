use crate::parser::ParserNode;
use crate::ops::OpContext;
use crate::table::{Store, Table};
use crate::result::SqlResult;

pub (super) fn eval(node: Option<ParserNode>, mut table: Table,
                    mut op_context: &OpContext, mut store: &Store) -> SqlResult<Table> {
    unimplemented!();
}