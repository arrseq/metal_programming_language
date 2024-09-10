use colored::Colorize;
use metal_programming_language::core::token;
use metal_programming_language::core::token::Token;

fn main() {
    let mut tokens = token::Iterator::from(include_str!("./variable/math.mtx"));
    
    println!("Colored tokens view");
    for token in tokens {
        let token_string = String::from(token);
        let painted = match token {
            Token::Space => token_string.white(),
            Token::Tab => token_string.white(),
            Token::Newline => token_string.white(),
            Token::Identifier(_) => token_string.red(),
            Token::Digit(_) => token_string.cyan(),
            Token::OpeningBracket => token_string.white(),
            Token::ClosingBracket => token_string.white(),
            Token::OpeningChevron => token_string.white(),
            Token::ClosingChevron => token_string.white(),
            Token::Path => token_string.bright_red(),
            Token::Macro => token_string.bright_yellow(),
            Token::Decimal => token_string.red(),
            Token::Stop => token_string.red(),
            Token::Separator => token_string.cyan(),
            Token::Equal => token_string.cyan(),
            Token::StringQuote => token_string.green(),
            Token::CharacterQuote => token_string.green(),
            Token::Escape => token_string.yellow(),
            Token::Comment => token_string.bright_black(),
            Token::Other(_) => token_string.magenta()
        };
        
        print!("{}", painted);
    }
}