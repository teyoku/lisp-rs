use std::error::Error;

#[derive(Debug)]
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
