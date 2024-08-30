use metal_programming_language::core::lexer::Tokens;

fn main() {
    let keywords = include_str!("./lexer_basic/keywords.mtx");
    let lexer = Tokens::from(keywords);
    
    let mut rebuilt = String::new();
    for token in lexer {
        println!("tk({:?})", token);
        rebuilt.push_str(&format!("{}", token));
    }
    
    assert_eq!(rebuilt.as_str(), keywords);
}