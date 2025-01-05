use xfparser::{Parsable, Parser};
use xfparser::error::Error;
use crate::node::name::Name;
use crate::token::Token;

pub const KEYWORDS: [(&str, Mode); 4] = [
    ("fun", Mode::Function),
    ("var", Mode::Variable),
    ("struct", Mode::Structure),
    ("pub", Mode::Public)
];

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Function,
    Variable,
    Structure,
    Public
}

pub fn internalize_keywords(parser: &mut Parser<Token>) {
    for keyword in KEYWORDS {
        parser.internalize(keyword.0, Token::Keyword(keyword.1)).unwrap_or_else(|_| panic!("Failed to tokenize keyword '{}'", keyword.0));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Keyword {
    pub mode: Mode
}

impl Parsable for Keyword {
    type Error = ();
    type Token = Token;
    type Data = ();

    fn parse(parser: &mut Parser<Self::Token>, _: &mut Self::Data) -> Result<Self, Error<Self::Error>> {
        let mut word = parser.parse::<Name>(&mut ()).unwrap_or_else(|_| panic!("could not parse word"));
        let must_be_fixed = word.slice().to_string();
        let mode = word.name.try_internalize(|_| panic!("could not recognize keyword '{}'", must_be_fixed)).unwrap().unwrap();
        let Token::Keyword(mode) = *mode else { unreachable!() };
        
        Ok(Self { mode })
    }
}