use crate::core::lexer::{Token, Tokens, Type};

#[test]
fn basic() {
    let source = "hello world!";
    let tokens = Tokens::from(source).collect::<Vec<Token>>();
    assert_eq!(tokens, vec![ 
        Token { start: 0,  end: 5,  r#type: Type::Identifier("hello".to_string()) },
        Token { start: 5,  end: 6,  r#type: Type::Space },
        Token { start: 6,  end: 11, r#type: Type::Identifier("world".to_string()) },
        Token { start: 11, end: 12, r#type: Type::BitwiseNotOperator },
    ])
}

#[test]
fn expression() {
    let source = "x = 5 + 10;";
    let tokens = Tokens::from(source).collect::<Vec<Token>>();
    assert_eq!(tokens, vec![
        Token { start: 0,  end: 1,  r#type: Type::Identifier("x".to_string()) },
        Token { start: 1,  end: 2,  r#type: Type::Space },
        Token { start: 2,  end: 3,  r#type: Type::AssignmentOperator },
        Token { start: 3,  end: 4,  r#type: Type::Space },
        Token { start: 4,  end: 5,  r#type: Type::Number(5) },
        Token { start: 5,  end: 6,  r#type: Type::Space },
        Token { start: 6,  end: 7,  r#type: Type::AddOperator },
        Token { start: 7,  end: 8,  r#type: Type::Space },
        Token { start: 8,  end: 10, r#type: Type::Number(10) },
        Token { start: 10, end: 11, r#type: Type::Semicolon },
    ])
}