use crate::tokenizer::{Token};
use std::collections::VecDeque;

pub mod rdp;
pub mod display;

#[derive(Debug, PartialEq, Eq, Clone)]
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
    Literal,
    Identifier,
    From,
    FromStatement,
    Where,
    GroupBy,
    OrderBy,
    OrderByStatement,
    Into,
    Limit,
    StarOperator,
}

#[derive(Clone)]
pub struct ParserNode {
    node_type: ParserNodeType,
    tokens: VecDeque<Token>, // in the case of certain operations / * +, a function call, etc
    children: VecDeque<ParserNode>,
}

impl ParserNode {
    pub fn new(node_type: ParserNodeType) -> Self {
        ParserNode {
            children: VecDeque::new(),
            node_type,
            tokens: VecDeque::new(),
        }
    }

    pub fn add_child(&mut self, node: ParserNode) {
        self.children.push_back(node);
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push_back(token);
    }

    pub fn get_tokens(&self) -> &VecDeque<Token> {
        &self.tokens
    }

    pub fn get_type(&self) -> &ParserNodeType {
        &self.node_type
    }

    pub fn release(self) -> (ParserNodeType, VecDeque<Token>, VecDeque<ParserNode>) {
        (self.node_type, self.tokens, self.children)
    }
}