use metal_programming_language::core::lexer::Tokens;
use metal_programming_language::core::parser;
use metal_programming_language::core::parser::traverser::Traverser;

struct Variable {
    name: String,
    initializer: u64
}

fn parse_variable(traverser: Traverser) -> Result<Variable, parser::Error> {
    todo!()
}

fn main() {
    let traverser = Traverser::from(include_str!("./traverser/math.mtx"));
}