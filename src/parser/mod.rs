use crate::tokenizer::Token;

pub mod rdp;
pub mod display;

#[derive(Debug)]
pub enum ParserNodeType {
    Query,
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
    FromStatement,
    Where,
    GroupBy,
    OrderBy,
    OrderByStatement,
    Into,
    Target,
}

pub struct ParserNode {
    node_type: ParserNodeType,
    tokens: Vec<Token>, // in the case of certain operations / * +, a function call, etc
    children: Vec<ParserNode>,
}

impl ParserNode {
    pub fn new(node_type: ParserNodeType) -> Self {
        ParserNode {
            children: vec!(),
            node_type,
            tokens: vec!(),
        }
    }

    pub fn add_child(&mut self, node: ParserNode) {
        self.children.push(node);
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn node_type(&self) -> &ParserNodeType {
        &self.node_type
    }

    pub fn get_children(&self) -> &Vec<ParserNode> {
        &self.children
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    pub fn set_type(&mut self, node_type: ParserNodeType) {
        self.node_type = node_type
    }


}