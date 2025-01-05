use xfparser::Parser;
use metal_programming_language::node::keyword::{internalize_keywords, Keyword, Mode};
use metal_programming_language::node::name::Name;
use metal_programming_language::node::space::Space;

fn main() {
    let source = "fun hello_world";
    let mut parser = Parser::new(source);

    internalize_keywords(&mut parser);

    let keyword = parser.parse::<Keyword>(&mut ()).unwrap();
    dbg!(keyword.mode);
    parser.parse::<Space>(&mut ()).unwrap();
    
    let name = parser.parse::<Name>(&mut ()).unwrap();
    let decl = match keyword.mode {
        Mode::Function => "function",
        Mode::Variable => "variable",
        Mode::Structure => "structure",
        Mode::Public => "public"
    };
    
    println!("the name after this '{}' keyword is '{}'", decl, &*name.name);
}