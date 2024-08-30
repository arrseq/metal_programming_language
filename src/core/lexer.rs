#[cfg(test)]
mod test;

use std::fmt;
use std::iter::{Enumerate, Peekable};
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // Whitespace.
    Space,                             // \s
    Tab,                               // \t
    Newline,                           // \n

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
    Identifier(String),                // alphanumeric and _
    LineCommentPrefix,                 // //
    DocumentationCommentPrefix,        // ///
    Other(String)
}

impl<'a> Type {
    pub const SYMBOL_MAPPINGS: [(&'a str, Type); 35] = [
        ("[", Type::OpeningBracket),
        ("]", Type::ClosingBracket),
        (";", Type::Semicolon),
        (":", Type::Colon),
        (",", Type::Comma),
        ("<", Type::OpeningChevron),
        (">", Type::ClosingChevron),
        ("\\", Type::Escape),
        ("#\\", Type::IdentifierEscape),
        ("+", Type::AddOperator),
        (".", Type::Point),
        ("-", Type::SubtractOperator),
        ("*", Type::MultiplyOperator),
        ("/", Type::DivideOperator),
        ("!", Type::BitwiseNotOperator),
        ("&", Type::BitwiseAndOperator),
        ("|", Type::BitwiseOrOperator),
        (">>", Type::BitwiseShiftRightOperator),
        ("<<", Type::BitwiseShiftLeftOperator),
        ("^", Type::BitwiseXorOperator),
        ("==", Type::EqualOperator),
        ("=", Type::AssignmentOperator),
        ("+=", Type::AndEqualOperator),
        ("-=", Type::SubtractEqualOperator),
        ("*=", Type::MultiplyEqualOperator),
        ("/=", Type::DivideEqualOperator),
        ("&=", Type::BitwiseAndEqualOperator),
        ("|=", Type::BitwiseOrEqualOperator),
        (">>=", Type::BitwiseShiftRightEqualOperator),
        ("<<=", Type::BitwiseShiftLeftEqualOperator),
        ("^=", Type::BitwiseXorEqualOperator),
        ("//", Type::LineCommentPrefix),
        ("///", Type::DocumentationCommentPrefix),
        ("'", Type::Quote),
        ("\"", Type::DoubleQuote)
    ];

    pub const STR_MAPPINGS: [(&'a str, Type); 4] = [
        ("true", Type::BoolLiteral(true)),
        ("false", Type::BoolLiteral(false)),
        ("var", Type::VariableKeyword),
        ("fun", Type::FunctionKeyword)
    ];
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Space => write!(f, " "),
            Type::Tab => write!(f, "\t"),
            Type::Newline => writeln!(f),
            Type::VariableKeyword => write!(f, "var"),
            Type::FunctionKeyword => write!(f, "fun"),
            Type::OpeningBracket => write!(f, "["),
            Type::ClosingBracket => write!(f, "]"),
            Type::Semicolon => write!(f, ";"),
            Type::Colon => write!(f, ":"),
            Type::Comma => write!(f, ","),
            Type::OpeningChevron => write!(f, "<"),
            Type::ClosingChevron => write!(f, ">"),
            Type::Escape => write!(f, "\\"),
            Type::IdentifierEscape => write!(f, "#\\"),
            Type::Quote => write!(f, "'"),
            Type::DoubleQuote => write!(f, "\""),
            Type::Point => write!(f, "."),
            Type::AddOperator => write!(f, "+"),
            Type::SubtractOperator => write!(f, "-"),
            Type::MultiplyOperator => write!(f, "*"),
            Type::DivideOperator => write!(f, "/"),
            Type::BitwiseNotOperator => write!(f, "!"),
            Type::BitwiseAndOperator => write!(f, "&"),
            Type::BitwiseOrOperator => write!(f, "|"),
            Type::BitwiseShiftRightOperator => write!(f, ">>"),
            Type::BitwiseShiftLeftOperator => write!(f, "<<"),
            Type::BitwiseXorOperator => write!(f, "^"),
            Type::EqualOperator => write!(f, "=="),
            Type::AssignmentOperator => write!(f, "="),
            Type::AndEqualOperator => write!(f, "+="),
            Type::SubtractEqualOperator => write!(f, "-="),
            Type::MultiplyEqualOperator => write!(f, "*="),
            Type::DivideEqualOperator => write!(f, "/="),
            Type::BitwiseAndEqualOperator => write!(f, "&="),
            Type::BitwiseOrEqualOperator => write!(f, "|="),
            Type::BitwiseShiftRightEqualOperator => write!(f, ">>="),
            Type::BitwiseShiftLeftEqualOperator => write!(f, "<<="),
            Type::BitwiseXorEqualOperator => write!(f, "^="),
            Type::BoolLiteral(b) => write!(f, "{}", b),
            Type::Number(n) => write!(f, "{}", n),
            Type::Identifier(s) => write!(f, "{}", s),
            Type::LineCommentPrefix => write!(f, "//"),
            Type::DocumentationCommentPrefix => write!(f, "///"),
            Type::Other(s) => write!(f, "{}", s)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub r#type: Type
}

#[derive(Debug, Clone)]
pub struct Tokens<'a>{
    max_index: usize,
    string: Peekable<Enumerate<Chars<'a>>>
}

impl<'a> Tokens<'a> {
    fn get_position(&mut self) -> usize {
        if let Some(next) = self.string.peek() { return next.0 }
        self.max_index
    }
}

impl<'a> From<&'a str> for Tokens<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            string: value
                .chars()
                .enumerate()
                .peekable(),
            max_index: value.chars()
                .count()
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.get_position();

        // match a white space character.
        let init_char = self.string.peek()?.1;
        let r#type = match init_char {
            ' ' => Some(Type::Space),
            '\t' => Some(Type::Tab),
            '\n' => Some(Type::Newline),
            _ => None
        };
        if let Some(ty) = r#type {
            self.string.next().unwrap();
            let end = self.get_position();
            return Some(Token { start, end, r#type: ty });
        }

        // match numbers
        if let Some((_, ch)) = self.string.peek() && let Some(digit) = ch.to_digit(10) {
            let mut output = digit as u64;
            self.string.next().unwrap();

            while let Some((_, ch)) = self.string.peek() && let Some(digit) = ch.to_digit(10) {
                output *= 10;
                output += digit as u64;
                self.string.next().unwrap();
            }

            let end = self.get_position();
            return Some(Token { start, end, r#type: Type::Number(output)})
        }

        let mut captured = String::new();
        
        // match non-alphabetical symbols.
        while let Some((_, ch)) = self.string.peek() {
            if ch.is_whitespace() || ch.is_alphabetic() { break }
            captured.push(*ch);
            self.string.next().unwrap();

            if let Some((_, ty)) = Type::SYMBOL_MAPPINGS.iter()
                .find(|p| p.0 == captured) {
                let end = self.get_position();
                return Some(Token { start, end, r#type: ty.clone() });
            }
        }
        if !captured.is_empty() {
            let end = self.get_position();
            return Some(Token { start, end, r#type: Type::Other(captured) });
        }

        // match keywords and identifiers.
        captured.clear();
        while let Some((_, ch)) = self.string.peek() {
            if !ch.is_alphabetic() && *ch != '_' { break }
            captured.push(*ch);
            self.string.next().unwrap();
        }
        if !captured.is_empty() {
            return if let Some(ty) = Type::STR_MAPPINGS.iter().find(|p| p.0 == captured) {
                let end = self.get_position();
                Some(Token { start, end, r#type: ty.1.clone() })
            } else {
                let end = self.get_position();
                Some(Token { start, end, r#type: Type::Identifier(captured) })
            }
        }

        None
    }
}