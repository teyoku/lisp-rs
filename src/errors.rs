use std::error::Error;

use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnknownSymbol(char),
    UnterminatedString(String),
    InvalidNumber(String),
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_msg = match self {
            LexerError::UnknownSymbol(ch) => &format!("Unknown symbol '{ch}'"),
            LexerError::UnterminatedString(s) => &format!("Unterminated string {s}"),
            LexerError::InvalidNumber(n) => &format!("Invalid number '{n}'"),
        };

        write!(f, "[Lexer Error] {error_msg}")
    }
}
impl Error for LexerError {}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedEof,
    UnexpectedClosingParen,
    ExpectedExpressionAfterQuote,
    TrailingTokens(Token),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_msg = match self {
            ParserError::UnexpectedEof => "Unexpected Eof",
            ParserError::UnexpectedClosingParen => "Unexpected closing paren",
            ParserError::ExpectedExpressionAfterQuote => "Expected expression after quote",
            ParserError::TrailingTokens(token) => &format!("Trailing tokens: '{token:?}'"),
        };

        write!(f, "[Lexer Error] {error_msg}")
    }
}
impl Error for ParserError {}
