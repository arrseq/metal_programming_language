use crate::core::lexer::{Token, Tokens};

#[test]
fn basic() {
    let source = "hello world!";
    let tokens = Tokens::from(source).collect::<Vec<Token>>();
    assert_eq!(tokens, vec![
        Token::Identifier(Box::<str>::from("hello")),
        Token::Space,
        Token::Identifier(Box::<str>::from("world")),
        Token::BitwiseNotOperator,
    ])
}

#[test]
fn number() {
    let source = "cats15";
    let tokens = Tokens::from(source).collect::<Vec<Token>>();
    assert_eq!(tokens, vec![
        Token::Identifier(Box::<str>::from("cats")),
        Token::Number(15)
    ])
}

#[test]
fn expression() {
    let source = "_x_x = 5 + 10;";
    let tokens = Tokens::from(source).collect::<Vec<Token>>();
    assert_eq!(tokens, vec![
        Token::Identifier(Box::<str>::from("_x_x")),
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