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
    tokens: Vec<Token>, // in the case of certain operations / * +, a function call, etc
    children: Vec<Box<ParserNode>>,
}

impl ParserNode {
    pub fn new(node_type: ParserNodeType) -> Self {
        ParserNode {
            children: vec!(),
            node_type,
            tokens: vec!(),
        }
    }

    pub fn add_child(&mut self, node: Box<ParserNode>) {
        self.children.push(node);
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

pub struct RecursiveDescentParser {
    root: Option<Box<ParserNode>>
}

type ParserResult = SqlResult<Box<ParserNode>>;

impl RecursiveDescentParser {

    pub fn parse(&self, tokens: &mut Tokens) -> ParserResult {
        self.parse_query(tokens)
    }

    fn parse_query(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Query));
        // always consume the next token
        let token = tokens.pop_front().unwrap();

        if !token.is("select") {
            return Err(SqlError::new("invalid select statement", Syntax))
        }

        // columns are required
        node.add_child(self.parse_columns(tokens)?);

        let optional_clauses = vec!("from", "where", "group by", "order by", "into");
        let mut current_index = 0;

        while let Some(token) = tokens.front() {
            let index = optional_clauses.iter().position(|s| { token.is(s) });

            match index {
                Some(i) => {
                    if i < current_index {
                        return Err(SqlError("select clauses out of order", Syntax));
                    }

                    current_index = i;

                    match optional_clauses[current_index] {
                        "from" => node.add_child(self.parse_from(tokens)?),
                        "where" => node.add_child(self.parse_where(tokens)?),
                        "group by" => node.add_child(self.parse_group_by(tokens)?),
                        "order by" => node.add_child(self.parse_order_by(tokens)?),
                        "into" => node.add_child(self.parse_into(tokens)?),
                        _ => ()
                    }
                }
                None => break
            }
        }

        Ok(node)
    }

    fn parse_columns(&self, tokens: &mut Tokens) -> ParserResult {

        let mut node = Box::new(ParserNode::new(ParserNodeType::Columns));

        node.add_child(self.parse_expression(tokens)?);

        while let Some(t) = tokens.front() {
            if t.is(",") {
                tokens.pop_front();
                node.add_child(self.parse_expression(tokens)?);
            } else {
                break;
            }
        }

        Ok(node)
    }

    // expression is used for readability but does not actually produce a node, making the walk a bit easier
    fn parse_expression(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Expression));

        node.add_child(self.parse_equality(tokens)?);

        Ok(node)
    }

    fn parse_equality(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Equality));

        node.add_child(self.parse_comparison(tokens)?);

        while let Some(t) = tokens.front() {
            if t.is("!=") || t.is("=") {
                node.add_token(tokens.pop_front().unwrap());
                node.add_child(self.parse_comparison(tokens)?);
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_comparison(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Comparison));

        node.add_child(self.parse_term(tokens)?);

        while let Some(t) = tokens.front() {
            if t.is(">") || t.is(">=") || t.is("<") || t.is("<=") {
                node.add_token(tokens.pop_front().unwrap()?);
                node.add_child(self.parse_term(tokens)?);
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_term(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Term));

        node.add_child(self.parse_factor(tokens)?);

        while let Some(t) = tokens.front() {
            if t.is("-") || t.is("+") {
                node.add_token(tokens.pop_front().unwrap()?);
                node.add_child(self.parse_factor(tokens)?);
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_factor(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Factor));

        node.add_child(self.parse_unary(tokens)?);

        while let Some(t) = tokens.front() {
            if t.is("/") || t.is("*") {
                node.add_token(tokens.pop_front().unwrap()?);
                node.add_child(self.parse_unary(tokens)?);
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_unary(&self, tokens: &mut Tokens) -> ParserResult {
        let mut node = Box::new(ParserNode::new(ParserNodeType::Unary));

        while let Some(t) = tokens.front() {
            if t.is("!") || t.is("-") {
                node.add_token(tokens.pop_front().unwrap()?);
                node.add_child(self.parse_unary(tokens)?);
            } else {
                node.add_child(self.parse_function(tokens)?);
            }
        }

        Ok(node)
    }

    fn parse_function(&self, tokens: &mut Tokens) -> ParserResult {

    }

    fn parse_primary(&self, tokens: &mut Tokens) -> ParserResult {}

    fn parse_from(&self, tokens: &mut Tokens) -> ParserResult {}

    fn parse_table(&self, tokens: &mut Tokens) -> ParserResult {}

    fn parse_where(&self, tokens: &mut Tokens) -> ParserResult {}

    fn parse_group_by(&self, tokens: &mut Tokens) -> ParserResult {}

    fn parse_order_by(&self, tokens: &mut Tokens) -> ParserResult {}

    fn parse_into(&self, tokens: &mut Tokens) -> ParserResult {}

    fn parse_target(&self, tokens: &mut Tokens) -> ParserResult {}

}





