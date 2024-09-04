#[cfg(test)]
mod test;

use std::marker::PhantomData;
use thiserror::Error;
use crate::core::lexer::Token;
use crate::core::parser::node::NodeTrait;
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

impl NodeTrait for Node {
    type Error = Error;

    fn parse(traverser: &mut Traverser) -> Result<Self, Self::Error> {
        let start = traverser.get_offset();
        traverser.consume_token(&Token::DoubleQuote).then_some(()).ok_or(Error::ExpectedOpeningQuote)?;

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
                    if escaping { return Err(Error::CannotEscape { symbol: traverser.next().unwrap() })}
                    token.to_string()
                }
            };

            traverser.next().unwrap();
            value += &*str_val;
        }

        traverser.consume_token(&Token::DoubleQuote).then_some(()).ok_or(Error::ExpectedClosingQuote)?;
        let end = traverser.get_offset();

        Ok(Self { 
            start, end, 
            value: value.into_boxed_str()
        })
    }

    fn start(&self) -> usize { self.start }
    fn end(&self) -> usize { self.end }
}