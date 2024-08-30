use crate::core::lexer::{Token, Tokens};

#[test]
fn basic() {
    let source = "hello world!";
    let tokens = Tokens::from(source).collect::<Vec<Token>>();
    assert_eq!(tokens, vec![
        Token::Identifier("hello".to_string()),
        Token::Space,
        Token::Identifier("world".to_string()),
        Token::BitwiseNotOperator,
    ])
}

#[test]
fn expression() {
    let source = "x = 5 + 10;";
    let tokens = Tokens::from(source).collect::<Vec<Token>>();
    assert_eq!(tokens, vec![
        Token::Identifier("x".to_string()),
        Token::Space,
        Token::AssignmentOperator,
        Token::Space,
        Token::Number(5),
        Token::Space,
        Token::AddOperator,
        Token::Space,
        Token::Number(10),
        Token::Semicolon,
    ])
}