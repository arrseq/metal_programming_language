
#[cfg(test)]
mod test;

use thiserror::Error;
use crate::core::lexer::Token;
use crate::core::parser::node;
use crate::core::parser::node::error;
use crate::core::parser::node::number::Number;
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

impl node::Node for Node {
    type Error = ();

    fn parse(traverser: &mut Traverser) -> Result<Self, error::Error<Self::Error>> {
        let start = traverser.offset();
        let escaped = traverser.try_consume_token(&Token::IdentifierEscape);
        
        // Number prefixed identifier.
        if escaped {
            let number = traverser.test_token_fast(|token| if let Token::Number())
        }
        
        let identifier = traverser
            .test_token_fast(|token| if let Token::Identifier(identifier) = token { Some(identifier) } else { None })
            .ok_or(error::Error::from_traverser(&traverser, ()))?;
        Ok(Self {
            start,
            end: traverser.offset(),
            value: identifier
        })
    }

    fn start(&self) -> usize { self.start }
    fn end(&self) -> usize { self.end }
}