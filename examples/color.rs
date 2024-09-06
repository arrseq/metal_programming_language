use colored::Colorize;
use metal_programming_language::core::lexer::{Tokens, Token};
use metal_programming_language::core::parser::node::{string, whitespace, Node};
use metal_programming_language::core::parser::traverser::Traverser;

fn colorize(sample: &str) {
    let tokens = Tokens::from(sample);

    for token in tokens {
        let tk_str = format!("{}", token);

        let (r, g, b) = match token {
            Token::FunctionKeyword
            | Token::VariableKeyword => (255, 93, 94),
            Token::OpeningBracket
            | Token::ClosingBracket => (200, 200, 200),
            Token::Number(_) => (76, 230, 253),
            Token::Space
            | Token::Tab
            | Token::NewLine => (240, 240, 240),
            Token::Semicolon
            | Token::Colon
            | Token::Comma => (180, 180, 180),
            Token::OpeningChevron
            | Token::ClosingChevron => (230, 76, 60),
            Token::AddOperator
            | Token::SubtractOperator
            | Token::MultiplyOperator
            | Token::DivideOperator => (255, 149, 0),
            Token::BitwiseAndOperator
            | Token::BitwiseOrOperator
            | Token::BitwiseXorOperator
            | Token::BitwiseNotOperator => (123, 104, 238),
            Token::EqualOperator
            | Token::AssignmentOperator
            | Token::AndEqualOperator
            | Token::SubtractEqualOperator
            | Token::MultiplyEqualOperator
            | Token::DivideEqualOperator
            | Token::BitwiseAndEqualOperator
            | Token::BitwiseOrEqualOperator
            | Token::BitwiseShiftRightEqualOperator
            | Token::BitwiseShiftLeftEqualOperator
            | Token::BitwiseXorEqualOperator => (255, 193, 7),
            Token::LineCommentPrefix
            | Token::DocumentationCommentPrefix => (150, 150, 150),
            _ => (255, 255, 255)
        };

        print!("{}", tk_str.truecolor(r, g, b));
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum IOS {
    Identifier,
    String
}

fn get_ident_or_string(tokens: &mut Traverser) -> Option<(IOS, Box<str>)> {
    let _ = whitespace::Node::parse(tokens);
    // try string
    let string = tokens.use_reverting(|tokens| {
        if let Ok(node) = string::Node::parse(tokens) { return Some(node) } 
        None
    });
    
    if let Some(string_node) = string { return Some((IOS::String, Box::<str>::from(string_node.value()))) }
    
    // try identifier.
    let ident = tokens.test_token_fast(|token| if let Token::Identifier(ident) = token { Some(ident) } else { None })?;
    Some((IOS::Identifier, ident))
}

fn print_strings() {
    let mut tokens = Traverser::from(include_str!("./color/strings.mtx"));
    loop {
        let Some(ios) = get_ident_or_string(&mut tokens) else { break };
        let str_val = match ios.0 {
            IOS::Identifier => ios.1.red(),
            IOS::String => ios.1.green()
        };
        
        println!("{}", str_val);
    }
}

fn main() {
    colorize(include_str!("./color/sample.mtx"));
    println!();
    colorize(include_str!("./color/random.mtx"));
    
    print_strings();
}