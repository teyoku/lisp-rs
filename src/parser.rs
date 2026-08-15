use crate::{errors::ParserError, lexer::Token};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    Symbol(String),
    Str(String),
    Quote(Box<Expr>),
    List(Vec<Expr>)
}

pub struct Parser {
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) -> Result<Expr, ParserError> {
        todo!()
    }
}