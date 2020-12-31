use crate::result::{SqlResult, SqlError};
use crate::parser::{ParserNode, ParserNodeType};
use crate::result::ErrorType::Syntax;

struct DividedAst {
    columns: ParserNode,
    from: Option<ParserNode>,
    group_by: Option<ParserNode>,
    where_: Option<ParserNode>,
    order_by: Option<ParserNode>,
    into: Option<ParserNode>,
}

fn divide(root: ParserNode) -> SqlResult<DividedAst> {
    if root.get_type() != ParserNodeType::Query {
        return Err(SqlError::new(format!("expected a query got a {:?}", root.get_type()).as_str(), Syntax));
    }



    Ok()
}