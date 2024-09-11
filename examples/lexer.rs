use std::cell::{Cell, RefCell};
use std::fmt::Debug;
use metal_programming_language::core::{node, token};
use metal_programming_language::core::node::{string, ErrorKind, Parsable, Traverser};
use inline_colorization::*;
use metal_programming_language::core::token::Token;

fn indent(line: usize) {
    print!("  {:02}  ", line);
}

fn print_error<Other: Debug + PartialEq>(error: node::Error<Other>, source: &str) {
    eprintln!("Parse error: Could not parse token {} because:", error.start_token);
    match error.kind {
        ErrorKind::ReachedEndForTokens { .. } => {}
        ErrorKind::ReachedEndForNodes { .. } => {}
        ErrorKind::UnexpectedToken { expectation, received } => eprintln!("  -   Expected either one of the tokens from {color_cyan}{expectation:?}{color_reset} but instead received {color_yellow}{received}{color_reset}"),
        ErrorKind::UnexpectedNode { .. } => {}
        ErrorKind::Other(other) => eprintln!("  Other: {:?}", other)
    }
    
    eprintln!("Input:");
    let stream = token::Iterator::from(source);
    
    indent(0);
    
    let mut line = 0usize;
    
    for (index, token) in stream.enumerate() {
        if index == error.start_token { print!("{color_red}{}{color_reset}", token) }
        else { print!("{}", token) }
        
        if let Token::Newline = token {
            line += 1;
            indent(line);
            
            continue
        }
    }
}

fn main() {
    let source = include_str!("./lexer/symbols.mtx");
    let mut tokens = node::Traverser::from(source);
    let result = string::Node::parse(&mut tokens).unwrap_err();
    print_error(result, source);
}