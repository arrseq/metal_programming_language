use metal_programming_language::core::parser::node::{string, whitespace, Node};

fn main() {
    let mut tokens = metal_programming_language::core::parser::traverser::Traverser::from(include_str!("./string/escape.mtx"));
    // dbg!(tokens.collect::<Vec<Token>>());
    loop {
        let _ = whitespace::Node::parse(&mut tokens);
        let result = string::Node::parse(&mut tokens).unwrap();
        println!("parsed {}", result.value())
    }
}