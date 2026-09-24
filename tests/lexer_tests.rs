use lisp_rs::lexer::{Lexer, TokenKind};

fn create_lexer<'a>(source: &'a str) -> Lexer<'a> {
    Lexer::new(source)
}

#[test]
fn test_add() {
    let tokens = create_lexer("(+ 1 2)").tokenize().unwrap();
    let kinds = tokens
        .iter()
        .map(|token| token.kind.clone())
        .collect::<Vec<TokenKind>>();

    assert_eq!(
        kinds,
        vec![
            TokenKind::LeftParen,
            TokenKind::Symbol("+".to_string()),
            TokenKind::Integer(1),
            TokenKind::Integer(2),
            TokenKind::RightParen,
        ]
    );
}

#[test]
fn test_area_of_cirlce() {
    let source = "
        (
            (define r 10)
            (define pi 314)
            (* pi (* r r))
        )
    ";
    let tokens = create_lexer(source).tokenize().unwrap();
    let kinds = tokens
        .iter()
        .map(|token| token.kind.clone())
        .collect::<Vec<TokenKind>>();

    assert_eq!(
        kinds,
        vec![
            TokenKind::LeftParen,
            TokenKind::LeftParen,
            TokenKind::Symbol("define".to_string()),
            TokenKind::Symbol("r".to_string()),
            TokenKind::Integer(10),
            TokenKind::RightParen,
            TokenKind::LeftParen,
            TokenKind::Symbol("define".to_string()),
            TokenKind::Symbol("pi".to_string()),
            TokenKind::Integer(314),
            TokenKind::RightParen,
            TokenKind::LeftParen,
            TokenKind::Symbol("*".to_string()),
            TokenKind::Symbol("pi".to_string()),
            TokenKind::LeftParen,
            TokenKind::Symbol("*".to_string()),
            TokenKind::Symbol("r".to_string()),
            TokenKind::Symbol("r".to_string()),
            TokenKind::RightParen,
            TokenKind::RightParen,
            TokenKind::RightParen,
        ]
    );
}

#[test]
fn test_string_and_comment() {
    let source = "
        \"hello!\" ;; This is a comment!
    ";
    let tokens = create_lexer(source).tokenize().unwrap();
    let kinds = tokens
        .iter()
        .map(|token| token.kind.clone())
        .collect::<Vec<TokenKind>>();

    assert_eq!(kinds, vec![TokenKind::String("hello!".to_string())])
}
