use crate::core::parser::node::{whitespace, Node};
use crate::core::parser::node::whitespace::{Symbol};
use crate::core::parser::traverser::Traverser;

#[test]
fn basic() {
    let mut tokens = Traverser::from(" \n\t\n ");
    let result = whitespace::Node::parse(&mut tokens).unwrap();
    
    assert_eq!(result, whitespace::Node {
        start: 0,
        end: 5,
        symbols: Box::from([
            Symbol::Space,
            Symbol::NewLine,
            Symbol::Tab,
            Symbol::NewLine,
            Symbol::Space
        ])
    })
}

#[test]
fn interruption() {
    let mut tokens = Traverser::from(" \n\thello\n ");
    let result = whitespace::Node::parse(&mut tokens).unwrap();

    assert_eq!(result, whitespace::Node {
        start: 0,
        end: 3,
        symbols: Box::from([
            Symbol::Space,
            Symbol::NewLine,
            Symbol::Tab
        ])
    })
}