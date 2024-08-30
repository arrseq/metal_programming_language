use colored::Colorize;
use metal_programming_language::core::lexer::{Tokens, Token};

fn colorize(sample: &str) {
    let tokens = Tokens::from(sample);

    for token in tokens {
        let tk_str = format!("{}", token);

        let (r, g, b) = match token {
            Token::FunctionKeyword
            | Token::VariableKeyword => (255, 93, 94),
            Token::OpeningBracket
            | Token::ClosingBracket => (200, 200, 200),
            Token::Number(_) => (76, 230, 253),
            Token::Space
            | Token::Tab
            | Token::Newline => (240, 240, 240),
            Token::Semicolon
            | Token::Colon
            | Token::Comma => (180, 180, 180),
            Token::OpeningChevron
            | Token::ClosingChevron => (230, 76, 60),
            Token::AddOperator
            | Token::SubtractOperator
            | Token::MultiplyOperator
            | Token::DivideOperator => (255, 149, 0),
            Token::BitwiseAndOperator
            | Token::BitwiseOrOperator
            | Token::BitwiseXorOperator
            | Token::BitwiseNotOperator => (123, 104, 238),
            Token::EqualOperator
            | Token::AssignmentOperator
            | Token::AndEqualOperator
            | Token::SubtractEqualOperator
            | Token::MultiplyEqualOperator
            | Token::DivideEqualOperator
            | Token::BitwiseAndEqualOperator
            | Token::BitwiseOrEqualOperator
            | Token::BitwiseShiftRightEqualOperator
            | Token::BitwiseShiftLeftEqualOperator
            | Token::BitwiseXorEqualOperator => (255, 193, 7),
            Token::LineCommentPrefix
            | Token::DocumentationCommentPrefix => (150, 150, 150),
            _ => (255, 255, 255)
        };

        print!("{}", tk_str.truecolor(r, g, b));
    }
}

fn main() {
    colorize(include_str!("./color/sample.mtx"));
    println!();
    colorize(include_str!("./color/random.mtx"));
}