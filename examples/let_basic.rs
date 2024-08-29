use metal_programming_language::parser::Parser;

fn main() {
    let source = String::from(include_str!("./let_basic/source.mtx"));
    let mut parser = Parser::new(source.chars().enumerate().peekable());
    
    dbg!(parser.parse_line_comment());
}