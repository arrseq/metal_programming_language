use std::cell::{Cell, RefCell};
use std::fmt::Debug;
use metal_programming_language::core::{node, token};
use metal_programming_language::core::node::{identifier, string, whitespace, Error, ErrorKind, NodeVariant, Parsable, Traverser};
use inline_colorization::*;
use metal_programming_language::core::node::string::Node;
use metal_programming_language::core::token::{Kind, Token};

fn indent(line: usize) {
    print!("  {:02}  ", line);
}

fn print_error<Other: Debug + PartialEq>(error: node::Error<Other>, source: &str) {
    eprintln!("Parse error: Could not parse token {} because:", error.start_token);
    match error.kind {
        ErrorKind::ReachedEndForTokens { tokens } => eprintln!("  -   Reached end when expecting the {color_cyan}{tokens:?}{color_reset} token"),
        ErrorKind::ReachedEndForNodes { .. } => eprintln!("  -   Reached end when expecting node"),
        ErrorKind::UnexpectedTokens { expectation, received } => eprintln!("  -   Expected either one of the tokens from {color_cyan}{expectation:?}{color_reset} but instead received {color_yellow}{received}{color_reset}"),
        ErrorKind::UnexpectedNodes { .. } => eprintln!("  -   Expected node"),
        ErrorKind::Other(other) => eprintln!("  Other: {:?}", other),
        _ => todo!()
    }
    
    eprintln!("Input:");
    let stream = token::Iterator::from(source);
    
    indent(0);
    
    let mut line = 0usize;
    
    for (index, token) in stream.enumerate() {
        if index == error.start_token { print!("{color_red}{}{color_reset}", token.kind()) }
        else { print!("{}", token.kind()) }
        
        if let Kind::NewLine = token.kind() {
            line += 1;
            indent(line);
            
            continue
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Red,
    White,
    Cyan
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DecoratedToken<'a> {
    color: Color,
    token: Token<'a>
}

fn try_node<'a, Node: Parsable<'a>, Other: Debug + PartialEq>(output: &mut Vec<NodeVariant<'a>>, tokens: &mut Traverser<'a>, mut parse: impl FnMut(&mut Traverser<'a>) -> Result<NodeVariant<'a>, Error<'a, Other>>) -> Result<(), Error<'a, Other>> {
    output.push(tokens.as_restorable(|restorable| parse(restorable))?);
    Ok(())
}

fn main() {
    let source = include_str!("./lexer/symbols.mtx");
    let mut tokens = node::Traverser::from(source);
    
    let mut nodes = Vec::new();
    loop {
        let _ = whitespace::Node::parse(&mut tokens);
        if try_node::<string::Node, <string::Node as Parsable>::Error>(&mut nodes, &mut tokens, |x| Ok(NodeVariant::String(string::Node::parse(x)?))).is_ok() { continue }
        if try_node::<identifier::Node, <identifier::Node as Parsable>::Error>(&mut nodes, &mut tokens, |x| Ok(NodeVariant::Identifier(identifier::Node::parse(x)?))).is_ok() { continue }
        break
    }
    
    let mut source_tokens = Traverser::from(source);
    let mut nodes = nodes.iter();
    let mut colored = Vec::new();
    
    for node in nodes {
        let (color, start, end) = match node {
            NodeVariant::WhiteSpace(n) => (Color::White, n.start_token(), n.end_token()),
            NodeVariant::String(n) => (Color::Red, n.start_token(), n.end_token()),
            NodeVariant::Identifier(id) => (Color::Cyan, id.start_token(), id.end_token())
        };
        
        let offset = start - source_tokens.token_offset();
        for _ in 0..offset { source_tokens.next().unwrap(); }
        let displacement = end - start;
        
        for _ in 0..displacement {
            let token = source_tokens.next().unwrap();
            colored.push(DecoratedToken { color: color.clone(), token })
        }
        
        // gets all tokens that fit between node.start()..node.end()
        
        // colored.push(DecoratedToken {
        //     color,
        //     token: node
        // })
    }
    
    for token in colored {
        match token.color {
            Color::Red => print!("{color_red}{}{color_reset}", token.token.kind()),
            Color::White => print!("{}", token.token.kind()),
            Color::Cyan => print!("{color_cyan}{}{color_reset}", token.token.kind())
        }
    }
}