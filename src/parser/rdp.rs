pub enum ParserNodeType {
    Query,
    Table,
    Columns,
    Expression,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Function,
    Primary,
    From,
    Where,
    GroupBy,
    OrderBy,
    Into,
    Target,
}

pub struct ParserNode {
    node_type: ParserNodeType,
    children: Vec<Box<ParserNode>>,
}

pub struct RecursiveDescentParser {

}

