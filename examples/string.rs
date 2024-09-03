use metal_programming_language::core::parser::node::NodeTrait;
use metal_programming_language::core::parser::node::string::Node;

fn main() {
    let mut tokens = metal_programming_language::core::parser::traverser::Traverser::from(r#""hello world""#);
    let result = Node::parse(&mut tokens);

    dbg!(result);
}