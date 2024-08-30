use std::fmt::format;
use std::iter::{Enumerate, Peekable};
use std::str::Chars;
use logos::Source;

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
    IntegerLiteral(i64),               // ...
    UnsignedIntegerLiteral(u64),       // ...
    FloatLiteral(f64),                 // ...
    CharLiteral(char),                 // "..."
    StringLiteral(String),             // '.'

    // Other.
    Identifier(String),                // alphanumeric and _
    LineCommentPrefix,                 // //
    DocumentationCommentPrefix         // ///
}

impl<'a> Type {
    pub const SYMBOL_MAPPINGS: [(&'a str, Type); 31] = [
        ("[", Type::OpeningBracket),
        ("]", Type::ClosingBracket),
        (";", Type::Semicolon),
        (":", Type::Colon),
        (",", Type::Comma),
        ("<", Type::OpeningChevron),
        (">", Type::ClosingChevron),
        ("\\", Type::Escape),
        ("+", Type::AddOperator),
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
        ("///", Type::DocumentationCommentPrefix)
    ];

    pub const STR_MAPPINGS: [(&'a str, Type); 4] = [
        ("true", Type::BoolLiteral(true)),
        ("false", Type::BoolLiteral(false)),
        ("var", Type::VariableKeyword),
        ("fun", Type::FunctionKeyword)
    ];
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

        // try to match a white space character.
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

        // try to match non-alphanumeric symbols.
        let mut captured = String::new();
        while let Some((_, ch)) = self.string.peek() {
            if ch.is_whitespace() || ch.is_alphanumeric() { break }
            captured.push(*ch);

            self.string.next().unwrap();
        }
        if let Some((_, ty)) = Type::SYMBOL_MAPPINGS.iter()
            .find(|p| p.0 == captured) {
            let end = self.get_position();
            return Some(Token { start, end, r#type: ty.clone() });
        }

        // try to match keywords.
        let mut captured = String::new();
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