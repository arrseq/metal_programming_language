#[cfg(test)]
mod test;

use thiserror::Error;
use crate::core::lexer::Token;
use crate::core::parser::{error, node, traverser};
use crate::core::parser::traverser::Traverser;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    start: usize,
    end: usize,
    value: Box<str>
}

impl Node {
    pub const fn value(&self) -> &str { &self.value }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Expected a quote to open the string")]
    ExpectedOpeningQuote,
    #[error("Expected a quote to close the string")]
    ExpectedClosingQuote,
    #[error("Cannot escape '{symbol}'")]
    CannotEscape { symbol: Token }
}

impl node::Node for Node {
    type Error = Error;

    fn parse(traverser: &mut Traverser) -> Result<Self, error::Error<Self::Error>> {
        let start = traverser.offset();
        traverser.consume_token(&Token::DoubleQuote).then_some(()).ok_or(error::Error::from_traverser(&traverser, Error::ExpectedOpeningQuote))?;

        let mut value = String::new();
        let mut escaping = false;

        while let Some(token) = traverser.peek() {
            let str_val = match token {
                Token::DoubleQuote => {
                    if !escaping { break }
                    escaping = false;
                    token.to_string()
                },
                Token::Escape => {
                    escaping = !escaping;
                    if escaping { 
                        traverser.next().unwrap();
                        continue
                    }
                    token.to_string()
                },
                _ => {
                    if escaping {
                        let symbol = traverser.next().unwrap();
                        return Err(error::Error::from_traverser(&traverser, Error::CannotEscape { symbol }))
                    }
                    token.to_string()
                }
            };

            traverser.next().unwrap();
            value += &*str_val;
        }

        traverser.consume_token(&Token::DoubleQuote).then_some(()).ok_or(error::Error::from_traverser(&traverser, Error::ExpectedClosingQuote))?;
        let end = traverser.offset();

        Ok(Self { 
            start, end, 
            value: Box::<str>::from(value)
        })
    }

    fn start(&self) -> usize { self.start }
    fn end(&self) -> usize { self.end }
}