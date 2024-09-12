use colored::Colorize;
use metal_programming_language::core::node::string::Node;
use metal_programming_language::core::token;
use metal_programming_language::core::token::Kind;

fn main() {
    let mut tokens = token::Iterator::from(include_str!("./variable/math.mtx"));
    
    println!("Colored tokens view");
    for token in tokens {
        let token_string = String::from(token);
        let painted = match token {
            Kind::Space => token_string.white(),
            Kind::Tab => token_string.white(),
            Kind::NewLine => token_string.white(),
            Kind::Identifier(i) => match i {
                "fun" 
                | "var"
                | "struct"
                | "derive"
                | "depend" => token_string.red(),
                _ => token_string.white()
            },
            Kind::Digit(_) => token_string.cyan(),
            Kind::OpeningBracket => token_string.white(),
            Kind::ClosingBracket => token_string.white(),
            Kind::OpeningChevron => token_string.white(),
            Kind::ClosingChevron => token_string.white(),
            Kind::Path => token_string.bright_red(),
            Kind::Macro => token_string.bright_yellow(),
            Kind::Decimal => token_string.red(),
            Kind::Stop => token_string.red(),
            Kind::Separator => token_string.cyan(),
            Kind::Equal => token_string.cyan(),
            Kind::StringQuote => token_string.green(),
            Kind::CharacterQuote => token_string.green(),
            Kind::Escape => token_string.yellow(),
            Kind::Comment => token_string.bright_black(),
            Kind::Other(_) => token_string.magenta()
        };
        
        print!("{}", painted);
    }
}