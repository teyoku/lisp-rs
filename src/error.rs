use std::fmt;

use crate::lexer::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum LispErrorKind {
    UnexpectedCharacter(char),
    UnterminatedString,
    InvalidNumber(String),

    UnexpectedToken {
        expected: String,
        found: String,
    },
    UnexpectedEndOfInput,
    UnclosedList,

    UndefinedVariable(String),
    NotCallable(String),

    WrongArity {
        function: String,
        expected: String,
        received: usize,
    },

    TypeError {
        expected: String,
        found: String,
    },

    DivisionByZero,
    InvalidSpecialForm(String),
    DuplicateParameter(String),

    Io(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LispError {
    pub kind: LispErrorKind,
    pub span: Option<Span>,
}

impl LispError {
    pub fn new(kind: LispErrorKind) -> Self {
        Self { kind, span: None }
    }

    pub fn at(kind: LispErrorKind, span: Span) -> Self {
        Self {
            kind,
            span: Some(span),
        }
    }
}

impl fmt::Display for LispError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(span) = &self.span {
            write!(f, "line {}, column {}: ", span.line, span.column)?;
        }

        match &self.kind {
            LispErrorKind::UnexpectedCharacter(character) => {
                write!(f, "unexpected character `{character}`")
            }
            LispErrorKind::UnterminatedString => write!(f, "unterminated string"),
            LispErrorKind::InvalidNumber(number) => write!(f, "invalid number `{number}`"),
            LispErrorKind::UnexpectedToken { expected, found } => {
                write!(f, "expected {expected}, found {found}")
            }
            LispErrorKind::UnexpectedEndOfInput => write!(f, "unexpected end of input"),
            LispErrorKind::UnclosedList => write!(f, "unclosed list"),
            LispErrorKind::UndefinedVariable(variable) => {
                write!(f, "undefined variable `{variable}`")
            }
            LispErrorKind::NotCallable(value) => {
                write!(f, "attempted to call non-function value `{value}`")
            }
            LispErrorKind::WrongArity {
                function,
                expected,
                received,
            } => write!(
                f,
                "expected {expected} arguments for `{function}`, received {received}"
            ),
            LispErrorKind::TypeError { expected, found } => {
                write!(f, "expected {expected}, found {found}")
            }
            LispErrorKind::DivisionByZero => write!(f, "division by zero"),
            LispErrorKind::InvalidSpecialForm(form) => {
                write!(f, "invalid special form `{form}`")
            }
            LispErrorKind::DuplicateParameter(parameter) => {
                write!(f, "duplicate parameter `{parameter}`")
            }
            LispErrorKind::Io(message) => write!(f, "I/O error: {message}"),
        }
    }
}

impl std::error::Error for LispError {}

impl From<std::io::Error> for LispError {
    fn from(value: std::io::Error) -> Self {
        Self::new(LispErrorKind::Io(value.to_string()))
    }
}
