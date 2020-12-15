use crate::result::SqlResult;
use crate::ops::math::Op;

pub struct Ast {
    op: Op,
    left: Box<AstNode>,
    right: Box<AstNode>,
}

pub enum AstNode {
    Ast(Box<Ast>),
    Result(SqlResult)
}