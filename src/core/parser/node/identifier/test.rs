use crate::core::parser::node::{identifier, Node};
use crate::core::parser::traverser::Traverser;

#[test]
fn basic() {
    let mut tokens = Traverser::from("hello_world");
    let result = identifier::Node::parse(&mut tokens).unwrap();
    assert_eq!(result, identifier::Node {
        start: 0,
        end: 1,
        value: Box::<str>::from("hello_world")
    });

    let mut tokens = Traverser::from("#\\15_cats");
    let result = identifier::Node::parse(&mut tokens).unwrap();
    assert_eq!(result, identifier::Node {
        start: 0,
        end: 1,
        value: Box::<str>::from("15_cats")
    });
}