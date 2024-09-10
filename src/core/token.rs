// # . , [ | ] = " < > \

use std::iter;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Token<'a> {
    #[default]
    Space,
    Tab,
    Newline,
    Identifier(&'a str),
    Digit(u8),

    OpeningBracket,
    ClosingBracket,
    OpeningChevron,
    ClosingChevron,

    Path,
    Macro,
    Decimal,
    Stop,
    Separator,
    Equal,
    StringQuote,
    CharacterQuote,
    Escape,
    Comment,

    Other(char)
}

impl<'a> From<Token<'a>> for String {
    fn from(value: Token<'a>) -> Self {
        match value {
            Token::Space => String::from(" "),
            Token::Tab => String::from("\t"),
            Token::Newline => String::from("\n"),
            Token::Identifier(identifier) => String::from(identifier),
            Token::Digit(digit) => format!("{}", digit),
            Token::OpeningBracket => String::from("["),
            Token::ClosingBracket => String::from("]"),
            Token::OpeningChevron => String::from("<"),
            Token::ClosingChevron => String::from(">"),
            Token::Path => String::from(":"),
            Token::Macro => String::from("#"),
            Token::Decimal => String::from(","),
            Token::Stop => String::from("."),
            Token::Separator => String::from("|"),
            Token::Equal => String::from("="),
            Token::StringQuote => String::from("'"),
            Token::CharacterQuote => String::from("\""),
            Token::Escape => String::from("\\"),
            Token::Comment => String::from("/"),
            Token::Other(other) => String::from(other)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mapping<'a> {
    pub character: char,
    pub token: Token<'a>
}

impl<'a> Token<'a> {
    pub const MAPPINGS: [Mapping<'a>; 16] = [
        Mapping { character: ' ',  token: Token::Space          },
        Mapping { character: '\t', token: Token::Tab            },
        Mapping { character: '\n', token: Token::Newline        },
        Mapping { character: '[',  token: Token::OpeningBracket },
        Mapping { character: ']',  token: Token::ClosingBracket },
        Mapping { character: '<',  token: Token::OpeningChevron },
        Mapping { character: '>',  token: Token::ClosingChevron },
        Mapping { character: ':',  token: Token::Path           },
        Mapping { character: '#',  token: Token::Macro          },
        Mapping { character: ',',  token: Token::Decimal        },
        Mapping { character: '.',  token: Token::Stop           },
        Mapping { character: '|',  token: Token::Separator      },
        Mapping { character: '=',  token: Token::Equal          },
        Mapping { character: '"',  token: Token::StringQuote    },
        Mapping { character: '\'', token: Token::CharacterQuote },
        Mapping { character: '/',  token: Token::Comment        }
    ];
}

#[derive(Debug, Clone)]
pub struct Iterator<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>
}

impl<'a> From<&'a str> for Iterator<'a> {
    fn from(value: &'a str) -> Self {
        Self { source: value, chars: iter::Iterator::peekable(value.char_indices()) }
    }
}

impl<'a> iter::Iterator for Iterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let peeked = self.chars.peek()?;
        if let Some(mapped) = Token::MAPPINGS.iter().find(|item| item.character == peeked.1) {
            let _ = self.chars.next();
            return Some(mapped.token)
        }
        if let Some(digit) = peeked.1.to_digit(10) {
            let _ = self.chars.next();
            return Some(Token::Digit(digit as u8))
        }
        
        let byte_start = peeked.0;
        let mut byte_end = byte_start;

        while let Some(character) = self.chars.peek() {
            if !character.1.is_alphabetic() && !character.1.is_alphanumeric() && character.1 != '_' { break }
            byte_end += character.1.len_utf8();
            let _ = self.chars.next();
        }

        if byte_end == byte_start { return Some(Token::Other(self.chars.next()?.1)) }
        Some(Token::Identifier(&self.source[byte_start..byte_end]))
    }
}