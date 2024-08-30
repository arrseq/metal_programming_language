use metal_programming_language::core::lexer::token::{Tokens, Type};

fn main() {
    let keywords = include_str!("./lexer_basic/keywords.mtx");
    let lexer = Tokens::from(keywords);
    
    for token in lexer { println!("tk({:?}) {}..{}", token.r#type, token.start, token.end) }
}