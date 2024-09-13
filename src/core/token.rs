use std::iter;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Kind<'a> {
    #[default]
    Space,
    Tab,
    NewLine,
    Identifier(&'a str),
    Digit(u8),

    OpeningBracket,
    ClosingBracket,
    OpeningChevron,
    ClosingChevron,

    Negate,
    IdentifierEscape,
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Token<'a> {
    kind: Kind<'a>,
    byte_length: usize
}

impl<'a> Token<'a> {
    pub fn kind(&self) -> &Kind<'a> { &self.kind }
    pub fn byte_length(&self) -> usize { self.byte_length }
}

impl<'a> std::fmt::Display for Kind<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Space => write!(f, " "),
            Kind::Tab => write!(f, "\t"),
            Kind::NewLine => write!(f, "\n"),
            Kind::Identifier(identifier) => write!(f, "{}", identifier),
            Kind::Digit(digit) => write!(f, "{}", digit),
            Kind::OpeningBracket => write!(f, "["),
            Kind::ClosingBracket => write!(f, "]"),
            Kind::OpeningChevron => write!(f, "<"),
            Kind::ClosingChevron => write!(f, ">"),
            Kind::Negate => write!(f, "-"),
            Kind::IdentifierEscape => write!(f, "_"),
            Kind::Path => write!(f, ":"),
            Kind::Macro => write!(f, "#"),
            Kind::Decimal => write!(f, ","),
            Kind::Stop => write!(f, "."),
            Kind::Separator => write!(f, "|"),
            Kind::Equal => write!(f, "="),
            Kind::StringQuote => write!(f, "\""),
            Kind::CharacterQuote => write!(f, "'"),
            Kind::Escape => write!(f, "\\"),
            Kind::Comment => write!(f, "/"),
            Kind::Other(other) => write!(f, "{}", other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mapping<'a> {
    pub character: char,
    pub token: Kind<'a>
}

impl<'a> Kind<'a> {
    pub const MAPPINGS: [Mapping<'a>; 19] = [
        Mapping { character: ' ',  token: Kind::Space            },
        Mapping { character: '\t', token: Kind::Tab              },
        Mapping { character: '\n', token: Kind::NewLine          },
        Mapping { character: '[',  token: Kind::OpeningBracket   },
        Mapping { character: ']',  token: Kind::ClosingBracket   },
        Mapping { character: '<',  token: Kind::OpeningChevron   },
        Mapping { character: '>',  token: Kind::ClosingChevron   },
        Mapping { character: '-',  token: Kind::Negate           },
        Mapping { character: '_',  token: Kind::IdentifierEscape },
        Mapping { character: ':',  token: Kind::Path             },
        Mapping { character: '#',  token: Kind::Macro            },
        Mapping { character: ',',  token: Kind::Decimal          },
        Mapping { character: '.',  token: Kind::Stop             },
        Mapping { character: '|',  token: Kind::Separator        },
        Mapping { character: '=',  token: Kind::Equal            },
        Mapping { character: '"',  token: Kind::StringQuote      },
        Mapping { character: '\'', token: Kind::CharacterQuote   },
        Mapping { character: '\\',  token: Kind::Escape          },
        Mapping { character: '/',  token: Kind::Comment          }
    ];
}

#[derive(Debug, Clone)]
pub struct Iterator<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>
}

impl<'a> Iterator<'a> {
    pub fn from_str(value: &'a str) -> Self { Self { source: value, chars: iter::Iterator::peekable(value.char_indices()) }}
    pub fn source(&self) -> &'a str { self.source }
}

impl<'a> iter::Iterator for Iterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let peeked = self.chars.peek()?;
        if let Some(mapped) = Kind::MAPPINGS.iter().find(|item| item.character == peeked.1) {
            let _ = self.chars.next();
            return Some(Token { kind: mapped.token, byte_length: mapped.character.len_utf8() })
        }
        if let Some(digit) = peeked.1.to_digit(10) {
            let character = self.chars.next().unwrap();
            return Some(Token { kind: Kind::Digit(digit as u8), byte_length: character.1.len_utf8() })
        }
        
        let byte_start = peeked.0;
        let mut byte_end = byte_start;

        while let Some(character) = self.chars.peek() {
            if !character.1.is_alphabetic() && !character.1.is_alphanumeric() && character.1 != '_' { break }
            byte_end += character.1.len_utf8();
            let _ = self.chars.next();
        }

        if byte_end == byte_start { 
            let character = self.chars.next()?.1;
            return Some(Token { kind: Kind::Other(character), byte_length: character.len_utf8() })
        }
        Some(Token { kind: Kind::Identifier(&self.source[byte_start..byte_end]), byte_length: byte_end - byte_start })
    }
}