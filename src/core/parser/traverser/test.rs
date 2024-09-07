use crate::core::parser::node::{error, number};
use crate::core::parser::node::number::Number;
use crate::core::parser::traverser::Traverser;

#[test]
fn parse_node() {
    let mut tokens = Traverser::from("10,100");
    let node: number::Node = tokens.try_parse_node().unwrap();
    
    assert_eq!(node, number::Node {
        start: 0,
        end: 3,
        value: Number::Float(10.100)
    });
    assert_eq!(tokens.offset(), 3);
}

#[test]
fn fail_parse_node() {
    let mut tokens = Traverser::from("a10,100");
    let error = tokens.try_parse_node::<number::Node>().unwrap_err();

    assert_eq!(error, error::Error::<number::Error> {
        kind: number::Error::ExpectedWholeNumber,
        end: 1,
    });
    assert_eq!(tokens.offset(), 0);
}