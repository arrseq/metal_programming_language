use std::fmt::Debug;
use thiserror::Error;
use crate::core::node;
use crate::core::node::{ErrorKind, NodeVariant, Parsable, Traverser};
use crate::core::token::Kind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Identifier<'a>(&'a str);

pub type Node<'a> = node::Node<Identifier<'a>>;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Expected an identifier")]
    ExpectedIdentifier,
    #[error("Expected an identifier but reached the end instead")]
    ReachedIdentifierEnd
}

impl<'a> Node<'a> {
    fn next_identifier(tokens: &mut Traverser<'a>) -> Result<&'a str, node::Error<Error>> {
        let Some(peeked) = tokens.peek() else { return Err(tokens.new_other_error(Error::ReachedIdentifierEnd)) };
        if let Kind::Identifier(identifier) = *peeked.kind() {
            let  _ = tokens.next().unwrap();
            return Ok(identifier);
        }

        let copied_peek = *peeked.kind();
        Err(tokens.new_other_error(Error::ExpectedIdentifier))
    }
}

impl<'a> Parsable<'a> for Node<'a> {
    type Error = Error;

    fn parse(tokens: &mut Traverser<'a>) -> Result<Self, node::Error<Self::Error>> {
        let start = tokens.token_offset();
        let identifier = Self::next_identifier(tokens)?;
        tokens.end(start, Identifier(identifier))
    }

    fn nodes(&self) -> Option<Vec<NodeVariant>> { None }
}