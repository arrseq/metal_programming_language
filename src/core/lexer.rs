#[cfg(test)]
mod test;

use std::fmt;
use std::iter::{Enumerate, Peekable};
use std::str::Chars;
use crate::core::parser::node::whitespace;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Whitespace.
    Space,                             // \s
    Tab,                               // \t
    NewLine,                           // \n

    // Keywords.
    VariableKeyword,                   // var
    FunctionKeyword,                   // fun

    // Symbols
    OpeningBracket,                    // [
    ClosingBracket,                    // ]
    Semicolon,                         // ;
    Colon,                             // :
    Comma,                             // ,
    OpeningChevron,                    // <
    ClosingChevron,                    // >
    Escape,                            // \
    IdentifierEscape,
    Quote,
    DoubleQuote,
    Point,

    // Operator
    AddOperator,                       // +
    SubtractOperator,                  // -
    MultiplyOperator,                  // *
    DivideOperator,                    // /

    BitwiseNotOperator,                // !
    BitwiseAndOperator,                // &
    BitwiseOrOperator,                 // |
    BitwiseShiftRightOperator,         // >>
    BitwiseShiftLeftOperator,          // <<
    BitwiseXorOperator,                // ^
    EqualOperator,                     // ==
    AssignmentOperator,                // =

    // Operator equal.
    AndEqualOperator,                  // +=
    SubtractEqualOperator,             // -=
    MultiplyEqualOperator,             // *=
    DivideEqualOperator,               // /=

    BitwiseAndEqualOperator,           // &=
    BitwiseOrEqualOperator,            // |=
    BitwiseShiftRightEqualOperator,    // >>=
    BitwiseShiftLeftEqualOperator,     // <<=
    BitwiseXorEqualOperator,           // ^=

    // Literal.
    BoolLiteral(bool),                 // true/false
    Number(u64),

    // Other.
    Identifier(Box<str>),              // alphanumeric and _
    LineCommentPrefix,                 // //
    DocumentationCommentPrefix,        // ///
    Other(String)
}

impl<'a> Token {
    pub const SYMBOL_MAPPINGS: [(&'a str, Token); 35] = [
        ("[", Token::OpeningBracket),
        ("]", Token::ClosingBracket),
        (";", Token::Semicolon),
        (":", Token::Colon),
        (",", Token::Comma),
        ("<", Token::OpeningChevron),
        (">", Token::ClosingChevron),
        ("\\", Token::Escape),
        ("#\\", Token::IdentifierEscape),
        ("+", Token::AddOperator),
        (".", Token::Point),
        ("-", Token::SubtractOperator),
        ("*", Token::MultiplyOperator),
        ("/", Token::DivideOperator),
        ("!", Token::BitwiseNotOperator),
        ("&", Token::BitwiseAndOperator),
        ("|", Token::BitwiseOrOperator),
        (">>", Token::BitwiseShiftRightOperator),
        ("<<", Token::BitwiseShiftLeftOperator),
        ("^", Token::BitwiseXorOperator),
        ("==", Token::EqualOperator),
        ("=", Token::AssignmentOperator),
        ("+=", Token::AndEqualOperator),
        ("-=", Token::SubtractEqualOperator),
        ("*=", Token::MultiplyEqualOperator),
        ("/=", Token::DivideEqualOperator),
        ("&=", Token::BitwiseAndEqualOperator),
        ("|=", Token::BitwiseOrEqualOperator),
        (">>=", Token::BitwiseShiftRightEqualOperator),
        ("<<=", Token::BitwiseShiftLeftEqualOperator),
        ("^=", Token::BitwiseXorEqualOperator),
        ("//", Token::LineCommentPrefix),
        ("///", Token::DocumentationCommentPrefix),
        ("'", Token::Quote),
        ("\"", Token::DoubleQuote)
    ];

    pub const STR_MAPPINGS: [(&'a str, Token); 4] = [
        ("true", Token::BoolLiteral(true)),
        ("false", Token::BoolLiteral(false)),
        ("var", Token::VariableKeyword),
        ("fun", Token::FunctionKeyword)
    ];
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Space => write!(f, " "),
            Token::Tab => write!(f, "\t"),
            Token::NewLine => writeln!(f),
            Token::VariableKeyword => write!(f, "var"),
            Token::FunctionKeyword => write!(f, "fun"),
            Token::OpeningBracket => write!(f, "["),
            Token::ClosingBracket => write!(f, "]"),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::OpeningChevron => write!(f, "<"),
            Token::ClosingChevron => write!(f, ">"),
            Token::Escape => write!(f, "\\"),
            Token::IdentifierEscape => write!(f, "#\\"),
            Token::Quote => write!(f, "'"),
            Token::DoubleQuote => write!(f, "\""),
            Token::Point => write!(f, "."),
            Token::AddOperator => write!(f, "+"),
            Token::SubtractOperator => write!(f, "-"),
            Token::MultiplyOperator => write!(f, "*"),
            Token::DivideOperator => write!(f, "/"),
            Token::BitwiseNotOperator => write!(f, "!"),
            Token::BitwiseAndOperator => write!(f, "&"),
            Token::BitwiseOrOperator => write!(f, "|"),
            Token::BitwiseShiftRightOperator => write!(f, ">>"),
            Token::BitwiseShiftLeftOperator => write!(f, "<<"),
            Token::BitwiseXorOperator => write!(f, "^"),
            Token::EqualOperator => write!(f, "=="),
            Token::AssignmentOperator => write!(f, "="),
            Token::AndEqualOperator => write!(f, "+="),
            Token::SubtractEqualOperator => write!(f, "-="),
            Token::MultiplyEqualOperator => write!(f, "*="),
            Token::DivideEqualOperator => write!(f, "/="),
            Token::BitwiseAndEqualOperator => write!(f, "&="),
            Token::BitwiseOrEqualOperator => write!(f, "|="),
            Token::BitwiseShiftRightEqualOperator => write!(f, ">>="),
            Token::BitwiseShiftLeftEqualOperator => write!(f, "<<="),
            Token::BitwiseXorEqualOperator => write!(f, "^="),
            Token::BoolLiteral(b) => write!(f, "{}", b),
            Token::Number(n) => write!(f, "{}", n),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::LineCommentPrefix => write!(f, "//"),
            Token::DocumentationCommentPrefix => write!(f, "///"),
            Token::Other(s) => write!(f, "{}", s)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tokens<'a>(Peekable<Chars<'a>>);

impl<'a> From<&'a str> for Tokens<'a> {
    fn from(value: &'a str) -> Self {
        Self(value
                .chars()
                .peekable())
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // match a white space character.
        let init_char = self.0.peek()?;
        let r#type = match init_char {
            ' ' => Some(Token::Space),
            '\t' => Some(Token::Tab),
            '\n' => Some(Token::NewLine),
            _ => None
        };
        if let Some(ty) = r#type {
            self.0.next().unwrap();
            return Some(ty);
        }

        // match numbers
        if let Some(ch) = self.0.peek() && let Some(digit) = ch.to_digit(10) {
            let mut output = digit as u64;
            self.0.next().unwrap();

            while let Some(ch) = self.0.peek() && let Some(digit) = ch.to_digit(10) {
                output *= 10;
                output += digit as u64;
                self.0.next().unwrap();
            }

            return Some(Token::Number(output))
        }

        let mut captured = String::new();
        
        // match non-alphabetical symbols.
        while let Some(ch) = self.0.peek() {
            if ch.is_whitespace() || ch.is_alphabetic() || *ch == '_' { break }
            captured.push(*ch);
            let _ = self.0.next();

            if let Some((_, ty)) = Token::SYMBOL_MAPPINGS.iter()
                .find(|p| p.0 == captured) {
                return Some(ty.clone());
            }
        }
        if !captured.is_empty() { return Some(Token::Other(captured)); }

        // match keywords and identifiers.
        captured.clear();
        while let Some(ch) = self.0.peek() {
            if !ch.is_alphabetic() && *ch != '_' && !ch.is_alphanumeric() { break }
            captured.push(*ch);
            self.0.next().unwrap();
        }
        if !captured.is_empty() {
            return if let Some(ty) = Token::STR_MAPPINGS.iter().find(|p| p.0 == captured) { Some(ty.1.clone()) } 
            else { Some(Token::Identifier(Box::<str>::from(captured))) }
        }

        None
    }
}

impl From<whitespace::Symbol> for Token {
    fn from(value: whitespace::Symbol) -> Self {
        match value {
            whitespace::Symbol::Space => Self::Space,
            whitespace::Symbol::Tab => Self::Tab,
            whitespace::Symbol::NewLine => Self::NewLine
        }
    }
}