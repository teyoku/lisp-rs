use lisp_rs::{
    ast::{Expr, ExprKind},
    error::LispErrorKind,
    lexer::{Lexer, Span},
    parser::Parser,
};

fn create_parser(source: &str) -> Parser {
    let tokens = Lexer::new(source).tokenize().unwrap();
    Parser::new(tokens)
}

#[derive(Debug, PartialEq)]
enum ParsedKind {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Symbol(String),
    List(Vec<ParsedKind>),
    Quote(Box<ParsedKind>),
}

fn without_spans(expression: &Expr) -> ParsedKind {
    match &expression.kind {
        ExprKind::Integer(value) => ParsedKind::Integer(*value),
        ExprKind::Float(value) => ParsedKind::Float(*value),
        ExprKind::Boolean(value) => ParsedKind::Boolean(*value),
        ExprKind::String(value) => ParsedKind::String(value.clone()),
        ExprKind::Symbol(value) => ParsedKind::Symbol(value.clone()),
        ExprKind::List(elements) => ParsedKind::List(elements.iter().map(without_spans).collect()),
        ExprKind::Quote(inner) => ParsedKind::Quote(Box::new(without_spans(inner))),
    }
}

#[test]
fn parses_empty_input() {
    assert_eq!(create_parser("   \n\t").parse().unwrap(), vec![]);
}

#[test]
fn parses_atomic_expressions() {
    let parsed = create_parser("42 -7 3.5 #t #f \"hello\" name")
        .parse()
        .unwrap();

    assert_eq!(
        parsed.iter().map(without_spans).collect::<Vec<_>>(),
        vec![
            ParsedKind::Integer(42),
            ParsedKind::Integer(-7),
            ParsedKind::Float(3.5),
            ParsedKind::Boolean(true),
            ParsedKind::Boolean(false),
            ParsedKind::String("hello".to_string()),
            ParsedKind::Symbol("name".to_string()),
        ]
    );
}

#[test]
fn parses_multiple_top_level_expressions() {
    let parsed = create_parser("one (two) 3").parse().unwrap();

    assert_eq!(
        parsed.iter().map(without_spans).collect::<Vec<_>>(),
        vec![
            ParsedKind::Symbol("one".to_string()),
            ParsedKind::List(vec![ParsedKind::Symbol("two".to_string())]),
            ParsedKind::Integer(3),
        ]
    );
}

#[test]
fn parses_empty_and_nested_lists() {
    let parsed = create_parser("(() (add 1 (mul 2 3)))").parse().unwrap();

    assert_eq!(
        parsed.iter().map(without_spans).collect::<Vec<_>>(),
        vec![ParsedKind::List(vec![
            ParsedKind::List(vec![]),
            ParsedKind::List(vec![
                ParsedKind::Symbol("add".to_string()),
                ParsedKind::Integer(1),
                ParsedKind::List(vec![
                    ParsedKind::Symbol("mul".to_string()),
                    ParsedKind::Integer(2),
                    ParsedKind::Integer(3),
                ]),
            ]),
        ])]
    );
}

#[test]
fn parses_quote_shorthand_and_keyword() {
    let shorthand = create_parser("'(a 1)").parse().unwrap();
    let keyword = create_parser("quote value").parse().unwrap();

    assert_eq!(
        shorthand.iter().map(without_spans).collect::<Vec<_>>(),
        vec![ParsedKind::Quote(Box::new(ParsedKind::List(vec![
            ParsedKind::Symbol("a".to_string()),
            ParsedKind::Integer(1),
        ])))]
    );
    assert_eq!(
        keyword.iter().map(without_spans).collect::<Vec<_>>(),
        vec![ParsedKind::Quote(Box::new(ParsedKind::Symbol(
            "value".to_string()
        )))]
    );
}

#[test]
fn list_span_starts_at_open_paren_and_ends_at_close_paren() {
    let parsed = create_parser("\n  (a\n    b)").parse().unwrap();

    assert_eq!(
        parsed[0].span,
        Span {
            start: 3,
            end: 12,
            line: 2,
            column: 3,
        }
    );
}

#[test]
fn rejects_unexpected_closing_parenthesis() {
    let error = create_parser(")").parse().unwrap_err();

    assert_eq!(
        error.kind,
        LispErrorKind::UnexpectedToken {
            expected: "expression".to_string(),
            found: ")".to_string(),
        }
    );
    assert_eq!(
        error.span,
        Some(Span {
            start: 0,
            end: 1,
            line: 1,
            column: 1,
        })
    );
}

#[test]
fn reports_unclosed_list_at_opening_parenthesis() {
    let error = create_parser("  (a 1").parse().unwrap_err();

    assert_eq!(error.kind, LispErrorKind::UnclosedList);
    assert_eq!(
        error.span,
        Some(Span {
            start: 2,
            end: 3,
            line: 1,
            column: 3,
        })
    );
}

#[test]
fn reports_end_of_input_after_quote() {
    let error = create_parser("'").parse().unwrap_err();

    assert_eq!(error.kind, LispErrorKind::UnexpectedEndOfInput);
    assert_eq!(error.span, None);
}
