use crate::result::{SqlResult, SqlError};
use crate::parser::{ParserNode, ParserNodeType};
use crate::result::ErrorType::Syntax;

pub (super) struct SplitAst {
    pub columns: Option<ParserNode>,
    pub limit: Option<ParserNode>,
    pub from: Option<ParserNode>,
    pub group_by: Option<ParserNode>,
    pub where_: Option<ParserNode>,
    pub order_by: Option<ParserNode>,
    pub into: Option<ParserNode>,
}

pub (super) fn split(root: ParserNode) -> SqlResult<SplitAst> {
    if root.get_type() != &ParserNodeType::Query {
        return Err(SqlError::new(format!("expected a query got a {:?}", root.get_type()).as_str(), Syntax));
    }

    let (_, _, children) = root.release();
    let mut divided_ast = SplitAst {
        columns: None,
        from: None,
        group_by: None,
        where_: None,
        order_by: None,
        into: None,
        limit: None,
    };

    children.into_iter().for_each(|node| {
        match node.get_type() {
            ParserNodeType::Columns => divided_ast.columns = Some(node),
            ParserNodeType::From => divided_ast.from = Some(node),
            ParserNodeType::GroupBy => divided_ast.group_by = Some(node),
            ParserNodeType::Where => divided_ast.where_ = Some(node),
            ParserNodeType::OrderBy => divided_ast.order_by = Some(node),
            ParserNodeType::Into => divided_ast.into = Some(node),
            ParserNodeType::Limit => divided_ast.limit = Some(node),
            _ => (),
        };
    });

    Ok(divided_ast)
}