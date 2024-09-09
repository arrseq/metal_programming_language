// # . , [ | ] = " < > \

use std::iter;
use std::iter::Peekable;
use std::str::Chars;

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

    Macro,
    Decimal,
    Stop,
    Separator,
    Equal,
    StringQuote,
    CharacterQuote,
    Escape
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mapping<'a> {
    pub character: char,
    pub token: Token<'a>
}

impl<'a> Token<'a> {
    pub const MAPPINGS: [Mapping<'a>; 13] = [
        Mapping { character: ' ',  token: Token::Space          },
        Mapping { character: '[',  token: Token::OpeningBracket },
        Mapping { character: ']',  token: Token::ClosingBracket },
        Mapping { character: '<',  token: Token::OpeningChevron },
        Mapping { character: '>',  token: Token::ClosingChevron },
        Mapping { character: '#',  token: Token::Macro          },
        Mapping { character: ',',  token: Token::Decimal        },
        Mapping { character: '.',  token: Token::Stop           },
        Mapping { character: '|',  token: Token::Separator      },
        Mapping { character: '=',  token: Token::Equal          },
        Mapping { character: '"',  token: Token::StringQuote    },
        Mapping { character: '\'', token: Token::CharacterQuote },
        Mapping { character: '\\', token: Token::Escape         }
    ];
}

#[derive(Debug, Clone)]
pub struct Iterator<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>
}

impl<'a> From<&'a str> for Iterator<'a> {
    fn from(value: &'a str) -> Self {
        Self { source: value, chars: iter::Iterator::peekable(value.chars()) }
    }
}

impl<'a> iter::Iterator for Iterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let peeked = self.chars.peek()?;
        Token::MAPPINGS.iter().find(|item| item.character == peeked)?;
        
        todo!()
    }
}