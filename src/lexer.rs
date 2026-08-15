use crate::errors::LexerError;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LParen, // (
    RParen, // )
    Symbol(String),
    Number(f64),
    String(String),
}

pub struct Lexer {
    chars: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => {
                    self.advance();
                }
                '(' => {
                    self.advance();
                    tokens.push(Token::LParen);
                }
                ')' => {
                    self.advance();
                    tokens.push(Token::RParen);
                }
                '"' => {
                    tokens.push(self.read_string()?);
                }
                _ => {
                    if ch.is_digit(10) || (ch == '-' && self.peek_next_is_digit()) {
                        tokens.push(self.read_number()?);
                    } else if is_symbol_start(ch) {
                        tokens.push(Token::Symbol(self.read_symbol()));
                    } else {
                        return Err(LexerError::UnknownSymbol(ch));
                    }
                }
            }
        }

        Ok(tokens)
    }

    // Next char is a digit
    fn peek_next_is_digit(&self) -> bool {
        self.chars
            .get(self.position + 1)
            .map_or(false, |c| c.is_digit(10))
    }

    // Read and parse string if current char is ' " '
    fn read_string(&mut self) -> Result<Token, LexerError> {
        self.advance(); // skip the "
        let mut string = String::new();

        while let Some(ch) = self.advance() {
            if ch == '"' {
                return Ok(Token::String(string));
            }
            string.push(ch);
        }

        Err(LexerError::UnterminatedString(string))
    }

    // Read and parse number
    fn read_number(&mut self) -> Result<Token, LexerError> {
        let mut number_str = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_digit(10) || ch == '.' || ch == '-' {
                number_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        let num = number_str
            .parse::<f64>()
            .map_err(|_| LexerError::InvalidNumber(number_str))?;
        Ok(Token::Number(num))
    }

    // Read and parse symbol (+, -, *, /, <, >. etc..)
    fn read_symbol(&mut self) -> String {
        let mut symbol_str = String::new();

        while let Some(ch) = self.peek() {
            if is_symbol_part(ch) {
                symbol_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        symbol_str
    }

    // Get current symbol
    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    // Get current symbol and move pos
    fn advance(&mut self) -> Option<char> {
        if self.position < self.chars.len() {
            let ch = self.chars[self.position];
            self.position += 1;
            Some(ch)
        } else {
            None
        }
    }
}

fn is_symbol_start(ch: char) -> bool {
    ch.is_alphabetic() || "+-*/?!=<>_".contains(ch)
}

fn is_symbol_part(ch: char) -> bool {
    is_symbol_start(ch) || ch.is_digit(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_1() -> Result<(), LexerError> {
        let mut lexer = Lexer::new("( + 1 2)");
        let tokens = lexer.tokenize()?;

        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("+".to_string()),
                Token::Number(1.0),
                Token::Number(2.0),
                Token::RParen
            ]
        );

        Ok(())
    }

    #[test]
    fn test_tokenize_2() -> Result<(), LexerError> {
        let mut lexer = Lexer::new("(define x (* 2 3.14))");
        let tokens = lexer.tokenize()?;

        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("x".to_string()),
                Token::LParen,
                Token::Symbol("*".to_string()),
                Token::Number(2.0),
                Token::Number(3.14),
                Token::RParen,
                Token::RParen
            ]
        );

        Ok(())
    }

    #[test]
    fn test_tokenize_3() -> Result<(), LexerError> {
        let mut lexer = Lexer::new("(define x \"Hello\")");
        let tokens = lexer.tokenize()?;

        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Symbol("define".to_string()),
                Token::Symbol("x".to_string()),
                Token::String("Hello".to_string()),
                Token::RParen
            ]
        );

        Ok(())
    }
}
