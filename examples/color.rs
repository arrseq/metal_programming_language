use colored::Colorize;
use metal_programming_language::core::lexer::{Tokens, Type};

fn colorize(sample: &str) {
    let tokens = Tokens::from(sample);

    for token in tokens {
        let tk_str = format!("{}", token.r#type);

        let (r, g, b) = match token.r#type {
            Type::FunctionKeyword
            | Type::VariableKeyword => (255, 93, 94),
            Type::OpeningBracket
            | Type::ClosingBracket => (200, 200, 200),
            Type::Number(_) => (76, 230, 253),
            Type::Space
            | Type::Tab
            | Type::Newline => (240, 240, 240),
            Type::Semicolon
            | Type::Colon
            | Type::Comma => (180, 180, 180),
            Type::OpeningChevron
            | Type::ClosingChevron => (230, 76, 60),
            Type::AddOperator
            | Type::SubtractOperator
            | Type::MultiplyOperator
            | Type::DivideOperator => (255, 149, 0),
            Type::BitwiseAndOperator
            | Type::BitwiseOrOperator
            | Type::BitwiseXorOperator
            | Type::BitwiseNotOperator => (123, 104, 238),
            Type::EqualOperator
            | Type::AssignmentOperator
            | Type::AndEqualOperator
            | Type::SubtractEqualOperator
            | Type::MultiplyEqualOperator
            | Type::DivideEqualOperator
            | Type::BitwiseAndEqualOperator
            | Type::BitwiseOrEqualOperator
            | Type::BitwiseShiftRightEqualOperator
            | Type::BitwiseShiftLeftEqualOperator
            | Type::BitwiseXorEqualOperator => (255, 193, 7),
            Type::LineCommentPrefix
            | Type::DocumentationCommentPrefix => (150, 150, 150),
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