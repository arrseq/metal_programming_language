#[cfg(test)]
mod test;

use thiserror::Error;
use crate::core::lexer::Token;
use crate::core::parser::node::NodeTrait;
use crate::core::parser::traverser::Traverser;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    start: usize,
    end: usize,
    value: String
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Expected a quote to open the string")]
    OpeningQuote,
    #[error("Expected a quote to close the string")]
    ClosingQuote
}

impl NodeTrait for Node {
    type Error = Error;

    fn parse(traverser: &mut Traverser) -> Result<Self, Self::Error> {
        let start = traverser.get_offset();
        traverser.consume_token(&Token::DoubleQuote).then_some(()).ok_or(Error::OpeningQuote)?;

        let mut value = String::new();

        // todo: Support escaping.
        while let Some(token) = traverser.peek() {
            let str_val = match token {
                Token::DoubleQuote => break,
                _ => token.to_string()
            };

            traverser.next().unwrap();
            value += &*str_val;
        }

        traverser.consume_token(&Token::DoubleQuote).then_some(()).ok_or(Error::ClosingQuote)?;
        let end = traverser.get_offset();

        Ok(Self { start, end, value })
    }

    fn get_start(&self) -> usize { self.start }
    fn get_end(&self) -> usize { self.end }
}