use crate::core::parser::node::Node;
use crate::core::parser::node::whitespace::{Node, Symbol};
use crate::core::parser::traverser::Traverser;

#[test]
fn basic() {
    let mut tokens = Traverser::from(" \n\t\n ");
    let result = Node::parse(&mut tokens).unwrap();
    
    assert_eq!(result, Node {
        start: 0,
        end: 5,
        symbols: Vec::from([
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
    let result = Node::parse(&mut tokens).unwrap();

    assert_eq!(result, Node {
        start: 0,
        end: 3,
        symbols: Vec::from([
            Symbol::Space,
            Symbol::NewLine,
            Symbol::Tab
        ])
    })
}