use xfparser::Parser;
use metal_programming_language::node::keyword::internalize_keywords;
use metal_programming_language::node::name::Name;

fn main() {
    let source = "hello_world";
    let mut parser = Parser::new(source);
    
    internalize_keywords(&mut parser);
    
    let name = parser.parse::<Name>(&mut ()).unwrap();
    dbg!(&*name.name);
}