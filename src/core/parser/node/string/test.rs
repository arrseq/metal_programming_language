use crate::core::parser::node::{string, Node};
use crate::core::parser::traverser::Traverser;

#[test]
fn basic() {
    let mut tokens = Traverser::from(r#""hello world""#);
    let result = string::Node::parse(&mut tokens).unwrap();
    
    assert_eq!(result, string::Node {
        start: 0,
        end: 5,
        value: String::from("hello world").into_boxed_str()
    });
}

#[test]
fn escaping() {
    let mut tokens = Traverser::from(r#""Quoted text: \"Text\"""#);
    let result = string::Node::parse(&mut tokens).unwrap();

    assert_eq!(result, string::Node {
        start: 0,
        end: 12,
        value: String::from(r#"Quoted text: "Text""#).into_boxed_str()
    });
}