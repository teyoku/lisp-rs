use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct ParseError {
    pub err: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Parse Error] {}", self.err)
    }
}

impl Error for ParseError {}
