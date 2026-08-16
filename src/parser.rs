use crate::{errors::ParseError, lexer::Token, object::Object};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        // Reversed tokens vec
        Self {
            tokens: tokens.into_iter().rev().collect(),
        }
    }

    pub fn parse(&mut self) -> Result<Object, ParseError> {
        let token = self.tokens.pop();
        if token != Some(Token::LParen) {
            return Err(ParseError {
                err: format!("Expected LParen found '{:?}'", token),
            });
        }

        let mut list = Vec::new();
        while let Some(token) = self.tokens.pop() {
            match token {
                Token::Keyword(k) => match k.as_str() {
                    "true" | "false" => list.push(Object::Bool(k == "true")),
                    _ => list.push(Object::Keyword(k)),
                },
                Token::BinaryOp(b) => list.push(Object::BinaryOp(b)),
                Token::Integer(n) => list.push(Object::Integer(n)),
                Token::Float(n) => list.push(Object::Float(n)),
                Token::String(s) => list.push(Object::String(s)),
                Token::Symbol(s) => list.push(Object::Symbol(s)),
                Token::LParen => {
                    self.tokens.push(Token::LParen);
                    let sub_list = self.parse()?;
                    list.push(sub_list);
                }
                Token::RParen => return Ok(Object::List(list)),
            }
        }

        Ok(Object::List(list))
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::*;

    fn create_parser(program: &str) -> Parser {
        let tokens = Lexer::new(program).tokenize();
        Parser::new(tokens)
    }

    #[test]
    fn test_add() -> Result<(), ParseError> {
        let list = create_parser("(+ 1 2)").parse()?;
        assert_eq!(
            list,
            Object::List(vec![
                Object::BinaryOp("+".to_string()),
                Object::Integer(1),
                Object::Integer(2)
            ])
        );

        Ok(())
    }

    #[test]
    fn test_area_of_a_circle() -> Result<(), ParseError> {
        let program = "
            (
                (define r 10)
                (define pi 314)
                (* pi (* r r))
            )
        ";
        let list = create_parser(program).parse()?;
        assert_eq!(
            list,
            Object::List(vec![
                Object::List(vec![
                    Object::Keyword("define".to_string()),
                    Object::Symbol("r".to_string()),
                    Object::Integer(10)
                ]),
                Object::List(vec![
                    Object::Keyword("define".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::Integer(314)
                ]),
                Object::List(vec![
                    Object::BinaryOp("*".to_string()),
                    Object::Symbol("pi".to_string()),
                    Object::List(vec![
                        Object::BinaryOp("*".to_string()),
                        Object::Symbol("r".to_string()),
                        Object::Symbol("r".to_string()),
                    ])
                ]),
            ])
        );

        Ok(())
    }
}
