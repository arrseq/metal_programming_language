use thiserror::Error;
use std::ops::Deref;
use xfparser::{Parsable, Parser, ParserString};
use xfparser::error::{Error, SyntaxError};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Name { 
    pub name: ParserString<Token>
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("The name is empty but it is escaped")]
    EmptyName
}

impl Parsable for Name {
    type Error = ParseError;
    type Token = Token;
    type Data = ();

    fn parse(parser: &mut Parser<Self::Token>, _: &mut Self::Data) -> Result<Self, Error<Self::Error>> {
        let escape = parser.expect_char('_').is_ok();
        let name = parser.parse_while(|character| character.is_alphanumeric() 
            || character == '_');
        
        match (name.is_empty(), escape) {
            (true, true) => panic!("escaped name must have a name"),
            (true, false) => panic!("no name parsed"),
            _ => {}
        };
        
        if name.starts_with(|character: char| character.is_numeric()) {
            panic!("name cannot start with a numeric value");
        }
        
        Ok(Self {
            name
        })
    }
}