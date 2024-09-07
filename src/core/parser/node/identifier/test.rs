use crate::core::parser::node::{error, identifier, number, Node};
use crate::core::parser::node::identifier::Component;
use crate::core::parser::node::number::Number;
use crate::core::parser::traverser::Traverser;

#[test]
fn basic() {
    let mut tokens = Traverser::from("hello_world");
    let result = identifier::Node::parse(&mut tokens).unwrap();
    assert_eq!(result, identifier::Node {
        start: 0,
        end: 1,
        value: Box::<[Component]>::from([ Component::Identifier(Box::<str>::from("hello_world")) ])
    });

    let mut tokens = Traverser::from("#\\15_cats");
    let result = identifier::Node::parse(&mut tokens).unwrap();
    assert_eq!(result, identifier::Node {
        start: 0,
        end: 3,
        value: Box::<[Component]>::from([ 
            Component::Number(number::Node {
                start: 1,
                end: 2,
                value: Number::UnsignedInteger(15)
            }),
            Component::Identifier(Box::<str>::from("_cats")) 
        ])
    });
}

#[test]
fn invalid() {
    let mut tokens = Traverser::from("");
    let result = identifier::Node::parse(&mut tokens).unwrap_err();
    assert_eq!(result, error::Error {
        kind: identifier::Error::Expected,
        end: 0
    });
}