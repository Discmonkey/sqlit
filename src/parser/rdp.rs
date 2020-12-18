use crate::tokenizer::{Token, Tokens};
use crate::result::{SqlError, SqlResult};
use crate::result::ErrorType::Syntax;

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
    token: Option<Token>, // in the case of certain operations / * +, a function call, etc
    children: Vec<Box<ParserNode>>,
}

impl ParserNode {
    pub fn new(node_type: ParserNodeType) -> Self {
        ParserNode {
            children: vec!(),
            node_type,
            token: None
        }
    }

    pub fn add_child(&mut self, node: Box<ParserNode>) {
        self.children.push(node);
    }
}

pub struct RecursiveDescentParser {
    root: Option<Box<ParserNode>>
}

type ParserResult = SqlResult<Box<ParserNode>>;

impl RecursiveDescentParser {

    pub fn parse(&self, &mut tokens: Tokens) -> ParserResult {
        self.parse_query(tokens)
    }

    fn parse_query(&self, &mut tokens: Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Query));
        // always consume the next token
        let token = tokens.pop_front().unwrap();

        if !token.is("select") {
            return Err(SqlError::new("invalid select statement", Syntax))
        }

        // columns are required
        node.add_child(self.parse_columns(tokens)?);

        let mut remaining = vec!("into", "order by", "group by", "where", "from");
        let mut next = tokens.front();

        // we have tokens remaining which means we need to try to match against some optional clauses
        while let Some(token) = next {
            while remaining.len() > 0 && !token.is(remaining.last().unwrap()) {
                remaining.pop();
            }

            if remaining.len() == 0 {
                return Err(SqlError::new(format!("{} is out of order", token.get_text()).as_str(), Syntax));
            }

            let mut clause = remaining.pop().unwrap();

            match clause {
                "into" => node.add_child(self.parse_into(tokens)?),
                "order by" => node.add_child(self.parse_order_by(tokens)?),
                "group by" => node.add_child(self.parse_group_by(tokens)?),
                "where" => node.add_child(self.parse_where(tokens)?),
                "from" => node.add_child(self.parse_from(tokens)?),
                _ => {}
            };

            next = tokens.front();
        }

        Ok(node)
    }
    fn parse_table(&self, &mut tokens: Tokens) -> ParserResult {}
    fn parse_columns(&self, &mut &tokens: Tokens) -> ParserResult {}
    fn parse_expression(&self, &mut &tokens: Tokens) -> ParserResult {}
    fn parse_equality(&self, &mut &tokens: Tokens) -> ParserResult {}
    fn parse_comparison(&self, &mut &tokens: Tokens) -> ParserResult {}
    fn parse_term(&self, &mut &tokens: Tokens) -> ParserResult {}
    fn parse_factor(&self, &mut tokens: Tokens) -> ParserResult {}
    fn parse_unary(&self, &mut tokens: Tokens) -> ParserResult {}
    fn parse_function(&self, &mut tokens: Tokens) -> ParserResult {}
    fn parse_primary(&self, &mut tokens: Tokens) -> ParserResult {}
    fn parse_from(&self, &mut tokens: Tokens) -> ParserResult {}
    fn parse_where(&self, &mut tokens: Tokens) -> ParserResult {}
    fn parse_group_by(&self, &mut tokens: Tokens) -> ParserResult {}
    fn parse_order_by(&self, &mut tokens: Tokens) -> ParserResult {}
    fn parse_into(&self, &mut tokens: Tokens) -> ParserResult {}
    fn parse_target(&self, &mut tokens: Tokens) -> ParserResult {}

}





