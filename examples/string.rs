use metal_programming_language::core::lexer::Token;
use metal_programming_language::core::parser::node::NodeTrait;
use metal_programming_language::core::parser::node::string::Node;

fn main() {
    let mut tokens = metal_programming_language::core::parser::traverser::Traverser::from(include_str!("./string/escape.mtx"));
    // dbg!(tokens.collect::<Vec<Token>>());
    let result = Node::parse(&mut tokens).unwrap();
    println!("parsed {}", result.value())
}