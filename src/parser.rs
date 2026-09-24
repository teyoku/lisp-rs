use std::{iter::Peekable, vec::IntoIter};

use crate::{
    ast::{Expr, ExprKind},
    error::{LispError, LispErrorKind},
    lexer::{Span, Token, TokenKind},
};

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, LispError> {
        let mut exprs = Vec::new();
        while self.tokens.peek().is_some() {
            exprs.push(self.parse_expr()?);
        }

        Ok(exprs)
    }

    fn parse_expr(&mut self) -> Result<Expr, LispError> {
        let token = self
            .tokens
            .next()
            .ok_or_else(|| LispError::new(LispErrorKind::UnexpectedEndOfInput))?;

        match token.kind {
            TokenKind::LeftParen => self.parse_list(token.span),
            TokenKind::RightParen => Err(LispError::at(
                LispErrorKind::UnexpectedToken {
                    expected: "expression".to_string(),
                    found: ")".to_string(),
                },
                token.span,
            )),
            TokenKind::Quote => {
                let inner = self.parse_expr()?;
                let total_span = Span {
                    start: token.span.start,
                    end: inner.span.end,
                    line: token.span.line,
                    column: token.span.column,
                };
                Ok(Expr {
                    kind: ExprKind::Quote(Box::new(inner)),
                    span: total_span,
                })
            }
            TokenKind::Integer(val) => Ok(Expr {
                kind: ExprKind::Integer(val),
                span: token.span,
            }),
            TokenKind::Float(val) => Ok(Expr {
                kind: ExprKind::Float(val),
                span: token.span,
            }),
            TokenKind::String(val) => Ok(Expr {
                kind: ExprKind::String(val),
                span: token.span,
            }),
            TokenKind::Boolean(val) => Ok(Expr {
                kind: ExprKind::Boolean(val),
                span: token.span,
            }),
            TokenKind::Symbol(val) => Ok(Expr {
                kind: ExprKind::Symbol(val),
                span: token.span,
            }),
        }
    }

    fn parse_list(&mut self, open_span: Span) -> Result<Expr, LispError> {
        let mut elements = Vec::new();

        loop {
            match self.tokens.peek() {
                Some(token) if token.kind == TokenKind::RightParen => {
                    let close_token = self.tokens.next().unwrap();
                    let total_span = Span {
                        start: open_span.start,
                        end: close_token.span.end,
                        line: open_span.line,
                        column: open_span.column,
                    };
                    return Ok(Expr {
                        kind: crate::ast::ExprKind::List(elements),
                        span: total_span,
                    });
                }
                Some(_) => {
                    elements.push(self.parse_expr()?);
                }
                None => {
                    return Err(LispError::at(LispErrorKind::UnclosedList, open_span));
                }
            }
        }
    }
}
