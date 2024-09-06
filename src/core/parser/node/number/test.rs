use crate::core::parser::node::error;
use crate::core::parser::node::{number, Node};
use crate::core::parser::node::number::Number;
use crate::core::parser::traverser::Traverser;

#[test]
fn integer() {
    let mut tokens = Traverser::from("-4096");
    let result = number::Node::parse(&mut tokens).unwrap();
    assert_eq!(result, number::Node {
        start: 0,
        end: 2,
        value: Number::Integer(-4096)
    });

    let mut tokens = Traverser::from("-1000");
    let result = number::Node::parse(&mut tokens).unwrap();
    assert_eq!(result, number::Node {
        start: 0,
        end: 2,
        value: Number::Integer(-1000)
    });

    let mut tokens = Traverser::from(" -477");
    let result = number::Node::parse(&mut tokens).unwrap_err();
    assert_eq!(result, error::Error {
        end: 0,
        kind: number::Error::ExpectedWholeNumber
    });
}

#[test]
fn unsigned_integer() {
    let mut tokens = Traverser::from("4096");
    let result = number::Node::parse(&mut tokens).unwrap();
    assert_eq!(result, number::Node {
        start: 0,
        end: 1,
        value: Number::UnsignedInteger(4096)
    });

    let mut tokens = Traverser::from("1000");
    let result = number::Node::parse(&mut tokens).unwrap();
    assert_eq!(result, number::Node {
        start: 0,
        end: 1,
        value: Number::UnsignedInteger(1000)
    });

    let mut tokens = Traverser::from(" 477");
    let result = number::Node::parse(&mut tokens).unwrap_err();
    assert_eq!(result, error::Error {
        end: 0,
        kind: number::Error::ExpectedWholeNumber
    });
}

#[test]
fn float() {
    let mut tokens = Traverser::from("10,9");
    let result = number::Node::parse(&mut tokens).unwrap();
    assert_eq!(result, number::Node {
        start: 0,
        end: 3,
        value: Number::Float(10.9)
    });

    let mut tokens = Traverser::from("40000000077,12345");
    let result = number::Node::parse(&mut tokens).unwrap();
    assert_eq!(result, number::Node {
        start: 0,
        end: 3,
        value: Number::Float(40000000077.12345)
    });

    let mut tokens = Traverser::from("4044,");
    let result = number::Node::parse(&mut tokens).unwrap_err();
    assert_eq!(result, error::Error {
        kind: number::Error::ExpectedFractionalNumber,
        end: 2
    });
}

#[test]
fn negative_float() {
    let mut tokens = Traverser::from("-10,9");
    let result = number::Node::parse(&mut tokens).unwrap();
    assert_eq!(result, number::Node {
        start: 0,
        end: 4,
        value: Number::Float(-10.9)
    });

    let mut tokens = Traverser::from("-40000000077,12345");
    let result = number::Node::parse(&mut tokens).unwrap();
    assert_eq!(result, number::Node {
        start: 0,
        end: 4,
        value: Number::Float(-40000000077.12345)
    });

    let mut tokens = Traverser::from("-4044,");
    let result = number::Node::parse(&mut tokens).unwrap_err();
    assert_eq!(result, error::Error {
        kind: number::Error::ExpectedFractionalNumber,
        end: 3
    });
}

#[test]
fn becomes_identifier() {
    let mut tokens = Traverser::from("#-4044_something");
    let result = number::Node::parse(&mut tokens).unwrap_err();
    assert_eq!(result, error::Error {
        kind: number::Error::FoundIdentifier,
        end: 2
    });

    let mut tokens = Traverser::from("#-4044atoms");
    let result = number::Node::parse(&mut tokens).unwrap_err();
    assert_eq!(result, error::Error {
        kind: number::Error::FoundIdentifier,
        end: 2
    });

    let mut tokens = Traverser::from("#atoms-4044");
    let result = number::Node::parse(&mut tokens).unwrap_err();
    assert_eq!(result, error::Error {
        kind: number::Error::ExpectedWholeNumber,
        end: 0
    });
}