use logos::Logos;

use crate::error::{LispError, LispErrorKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Errors produced while tokenizing Lisp source.
pub enum LexError {
    #[default]
    UnexpectedCharacter,
    UnterminatedString,
    InvalidNumber,
}

#[derive(Logos, Debug, Clone, PartialEq)]
/// A lexical token recognized by the Lisp lexer.
#[logos(error = LexError)]
#[logos(skip r"[ \t\n\r\f]+")]
#[logos(skip(r";[^\n\r]*", allow_greedy = true))]
pub enum TokenKind {
    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("'")]
    #[token("quote")]
    Quote,

    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<i64>().map_err(|_| LexError::InvalidNumber))]
    Integer(i64),

    #[regex(r"-?[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().map_err(|_| LexError::InvalidNumber))]
    Float(f64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let slice = lex.slice();
        slice[1..slice.len() - 1].to_string()
    })]
    String(String),

    #[token("#t", |_| true)]
    #[token("#f", |_| false)]
    Boolean(bool),

    #[regex(r"[a-zA-Z+\-*/<>=?!_][a-zA-Z0-9+\-*/<>=?!_]*", |lex| lex.slice().to_string(), priority = 1)]
    Symbol(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Location of a token in the source text.
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
/// A token and its location in the source text.
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

/// Tokenizes Lisp source code into a sequence of tokens.
pub struct Lexer<'a> {
    source: &'a str,
}

impl<'a> Lexer<'a> {
    /// Creates a `Lexer` for the provided source string.
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Tokenizes the source, returning the first lexical error encountered.
    pub fn tokenize(self) -> Result<Vec<Token>, LispError> {
        let mut tokens = Vec::new();
        let mut logos_lexer = TokenKind::lexer(self.source);

        let mut current_line = 1;
        let mut current_column = 1;
        let mut last_byte_pos = 0;

        while let Some(token_res) = logos_lexer.next() {
            let byte_range = logos_lexer.span();

            let leading_chunk = &self.source[last_byte_pos..byte_range.start];
            for ch in leading_chunk.chars() {
                if ch == '\n' {
                    current_line += 1;
                    current_column = 1;
                } else {
                    current_column += 1;
                }
            }

            let token_span = Span {
                start: byte_range.start,
                end: byte_range.end,
                line: current_line,
                column: current_column,
            };

            match token_res {
                Ok(kind) => {
                    tokens.push(Token {
                        kind,
                        span: token_span,
                    });
                }
                Err(lex_err) => {
                    let kind = match lex_err {
                        LexError::UnexpectedCharacter => {
                            let invalid_char =
                                self.source[byte_range].chars().next().unwrap_or(' ');
                            LispErrorKind::UnexpectedCharacter(invalid_char)
                        }
                        LexError::UnterminatedString => LispErrorKind::UnterminatedString,
                        LexError::InvalidNumber => {
                            LispErrorKind::InvalidNumber(logos_lexer.slice().to_string())
                        }
                    };

                    return Err(LispError::at(kind, token_span));
                }
            }

            last_byte_pos = byte_range.end;
        }

        Ok(tokens)
    }
}
