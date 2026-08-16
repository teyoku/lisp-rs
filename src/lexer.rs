use std::{collections::HashSet, str::Chars};

pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
    Float(f64),
    String(String),
    BinaryOp(String),
    Keyword(String),
}

pub struct Lexer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
    keywords: HashSet<&'a str>,
    binary_ops: HashSet<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();

        let keywords: HashSet<&str> =
            vec!["define", "print", "lambda", "if", "cond", "else", "let"]
                .into_iter()
                .collect();

        let binary_ops: HashSet<char> = vec!['+', '-', '*', '/', '%', '<', '>', '=', '&', '|']
            .into_iter()
            .collect();

        Self {
            input: chars,
            current_char,
            keywords,
            binary_ops,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        todo!()
    }

    fn advance(&mut self) -> Option<char> {
        self.current_char = self.input.next();
        self.current_char
    }
}
