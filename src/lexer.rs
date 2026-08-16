use std::{collections::HashSet, str::Chars};

#[derive(Debug, Clone, PartialEq)]
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

        let keywords: HashSet<&str> = vec![
            "define", "print", "lambda", "if", "cond", "else", "let", "true", "false",
        ]
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

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(t) = self.next_token() {
            tokens.push(t);
        }

        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        self.eat_whitespace();

        match self.current_char? {
            '(' => {
                self.advance();
                Some(Token::LParen)
            }
            ')' => {
                self.advance();
                Some(Token::RParen)
            }
            '"' => Some(Token::String(self.read_string())),
            c if c.is_numeric() => {
                // Tokenize number (integer or float)
                let val = self.read_number();
                if val.contains('.') {
                    Some(Token::Float(val.parse().unwrap()))
                } else {
                    Some(Token::Integer(val.parse().unwrap()))
                }
            }
            c if c.is_alphabetic() || self.binary_ops.contains(&c) => {
                // Tokenize symbol, binary operator or keyword
                let sym = self.read_symbol();
                if self.keywords.contains(sym.as_str()) {
                    Some(Token::Keyword(sym))
                } else if self.binary_ops.contains(&sym.chars().next().unwrap()) {
                    Some(Token::BinaryOp(sym))
                } else {
                    Some(Token::Symbol(sym))
                }
            }
            _ => None,
        }
    }

    fn eat_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn read_string(&mut self) -> String {
        let mut string = String::new();
        self.advance(); // Skip the opening quote

        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance(); // Skip the closing quote
                break;
            }
            string.push(c);
            self.advance();
        }

        string
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();
        while let Some(c) = self.current_char {
            if !c.is_numeric() && c != '.' {
                break;
            }
            number.push(c);
            self.advance();
        }

        number
    }

    fn read_symbol(&mut self) -> String {
        let mut symbol = String::new();
        while let Some(c) = self.current_char {
            if c.is_whitespace() || c == '(' || c == ')' || c == '\'' {
                break;
            }
            symbol.push(c);
            self.advance();
        }

        symbol
    }

    fn advance(&mut self) -> Option<char> {
        self.current_char = self.input.next();
        self.current_char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let tokens = Lexer::new("(+ 1 2)").tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::BinaryOp("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen
            ]
        );
    }

    #[test]
    fn test_area_of_cirlce() {
        let program = "
            (
                (define r 10)
                (define pi 314)
                (* pi (* r r))
            )
        ";
        let tokens = Lexer::new(program).tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::LParen,
                Token::Keyword("define".to_string()),
                Token::Symbol("r".to_string()),
                Token::Integer(10),
                Token::RParen,
                Token::LParen,
                Token::Keyword("define".to_string()),
                Token::Symbol("pi".to_string()),
                Token::Integer(314),
                Token::RParen,
                Token::LParen,
                Token::BinaryOp("*".to_string()),
                Token::Symbol("pi".to_string()),
                Token::LParen,
                Token::BinaryOp("*".to_string()),
                Token::Symbol("r".to_string()),
                Token::Symbol("r".to_string()),
                Token::RParen,
                Token::RParen,
                Token::RParen,
            ]
        );
    }
}
