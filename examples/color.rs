use colored::Colorize;
use metal_programming_language::core::lexer::{Tokens, Token};
use metal_programming_language::core::parser::node::{string, whitespace, Node};
use metal_programming_language::core::parser::traverser::Traverser;

#[derive(Debug, Clone, Copy, PartialEq)]
enum IOS {
    Identifier,
    String
}

fn get_ident_or_string(tokens: &mut Traverser) -> Option<(IOS, Box<str>, usize, usize)> {
    let _ = whitespace::Node::parse(tokens);
    // try string
    let string = tokens.use_reverting(|tokens| {
        if let Ok(node) = string::Node::parse(tokens) { return Some(node) } 
        None
    });
    
    if let Some(string_node) = string { return Some((IOS::String, Box::<str>::from(string_node.value()), string_node.start(), string_node.end())) }
    
    // try identifier.
    let ident = tokens.test_token_fast(|token| if let Token::Identifier(ident) = token { Some(ident) } else { None })?;
    Some((IOS::Identifier, ident, todo!(), todo!()))
}

fn print_strings() {
    let source = include_str!("./color/strings.mtx");
    let mut tokens = Traverser::from(source);
    loop {
        let Some(ios) = get_ident_or_string(&mut tokens) else { break };
        let sub_str = 
        let str_val = match ios.0 {
            IOS::Identifier => ios.1.red(),
            IOS::String => ios.1.green()
        };
        
        println!("{}", str_val);
    }
}

fn main() {
    print_strings();
}