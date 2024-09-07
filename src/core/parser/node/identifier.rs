
#[cfg(test)]
mod test;

use thiserror::Error;
use crate::core::lexer::Token;
use crate::core::parser::node;
use crate::core::parser::node::{error, number};
use crate::core::parser::traverser::Traverser;

#[derive(Debug, Clone, PartialEq)]
pub enum Component {
    Identifier(Box<str>),
    Number(number::Node)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    start: usize,
    end: usize,
    value: Box<[Component]>
}

impl Node {
    pub const fn value(&self) -> &[Component] { &self.value }
    
    fn parse_component(traverser: &mut Traverser) -> Result<Component, error::Error<<Self as node::Node>::Error>> {
        let identifier_token = traverser.test_token(|token| if let Token::Identifier(identifier) = token { Some(identifier) } else { None });
        if let Some(identifier) = identifier_token { return Ok(Component::Identifier(identifier)) }
        
        let number_token = traverser
            .try_parse_node::<number::Node>()
            .map_err(|number_error| error::Error::from_traverser(traverser, Error::Number(number_error)))?;
        Ok(Component::Number(number_token))
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("An error happened when parsing a number at the start of the identifier")]
    Number(error::Error<number::Error>),
    #[error("Expected an identifier token or number")]
    Expected
}

impl node::Node for Node {
    type Error = Error;

    fn parse(traverser: &mut Traverser) -> Result<Self, error::Error<Self::Error>> {
        let start = traverser.offset();
        let _ = traverser.try_consume_token(&Token::IdentifierEscape);
        let mut components = Vec::new();
        
        loop {
            let component = match Self::parse_component(traverser) {
                Ok(component) => component,
                Err(error) => match error.kind {
                    Error::Expected => break,
                    _ => if !components.is_empty() { return Err(error) } else { break }
                }
            };
            
            components.push(component);
        }
        
        if components.is_empty() { return Err(error::Error::from_traverser(traverser, Error::Expected)) }
        
        Ok(Self {
            start,
            end: traverser.offset(),
            value: components.into_boxed_slice()
        })
    }

    fn start(&self) -> usize { self.start }
    fn end(&self) -> usize { self.end }
}