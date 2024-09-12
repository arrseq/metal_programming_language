pub mod identifier;
pub mod string;
pub mod whitespace;

#[cfg(test)]
mod test;

use std::fmt::Debug;
use std::iter::Peekable;
use thiserror::Error;
use crate::core::{node, token};
use crate::core::token::{Kind, Token};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeKind {
    WhiteSpace,
    String,
    Identifier
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeVariant<'a> {
    WhiteSpace(whitespace::Node),
    String(string::Node<'a>),
    Identifier(identifier::Node<'a>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node<Data: Debug + Clone + PartialEq> {
    pub(self) start_token: usize,
    pub(self) end_token: usize,
    pub(self) data: Data
}

impl<Data: Debug + Clone + PartialEq> Node<Data> {
    pub const fn start_token(&self) -> usize { self.start_token }
    pub const fn end_token(&self) -> usize { self.end_token }
    pub const fn data(&self) -> &Data { &self.data }
}

#[derive(Debug, Error, PartialEq)]
pub enum ErrorKind<'a, Other: Debug + PartialEq> {
    #[error("Reached the end of the parsable stream when {tokens:?} was expected")]
    ReachedEndForTokens { tokens: Box<[Kind<'a>]> },
    #[error("Reached the end of the parsable stream when {nodes:?} was expected")]
    ReachedEndForNodes { nodes: Box<[NodeKind]> },
    #[error("Reached the end of the parsable stream when {token} was expected")]
    ReachedEndForToken { token: Kind<'a> },
    #[error("Reached the end of the parsable stream when {node:?} was expected")]
    ReachedEndForNode { node: NodeKind },
    #[error("Expected token {expectation:?} but received {received} instead")]
    UnexpectedTokens { expectation: Box<[Kind<'a>]>, received: Kind<'a> },
    #[error("Expected node {expectation:?} but received {received:?} instead")]
    UnexpectedNodes { expectation: Box<[NodeKind]>, received: NodeKind },
    #[error("Expected token {expectation:?} but received {received} instead")]
    UnexpectedToken { expectation: Kind<'a>, received: Kind<'a> },
    #[error("Expected node {expectation:?} but received {received:?} instead")]
    UnexpectedNode { expectation: NodeKind, received: NodeKind },
    #[error("Expected an identifier but received {received} instead")]
    ExpectedIdentifier { received: Kind<'a> },
    #[error("Expected an identifier but reached the end instead")]
    ReachedIdentifierEnd,
    #[error("Failed to parse node due to other reason")]
    Other(Other)
}

#[derive(Debug, Error, PartialEq)]
#[error("Failed to parse a node")]
pub struct Error<'a, Other: Debug + PartialEq> {
    pub kind: ErrorKind<'a, Other>,
    pub start_token: usize
}

#[derive(Debug, Clone)]
pub struct Traverser<'a> {
    tokens: Peekable<token::Iterator<'a>>,
    token_offset: usize,
    source: &'a str,
    string_byte_offset: usize
}

impl<'a> Traverser<'a> {
    pub const fn token_offset(&self) -> usize { self.token_offset }
    pub const fn byte_offset(&self) -> usize { self.string_byte_offset }
    pub const fn source(&self) -> &'a str { self.source }

    pub const fn new_error<Other: Debug + PartialEq>(&self, kind: ErrorKind<'a, Other>) -> Error<'a, Other> {
        Error {
            kind,
            start_token: self.token_offset,
        }
    }

    pub const fn new_other_error<Other: Debug + PartialEq>(&self, other: Other) -> Error<'a, Other> {
        self.new_error(ErrorKind::Other(other))
    }

    pub const fn end<Other: Debug + PartialEq, Data: Debug + Clone + PartialEq>(&self, start: usize, data: Data) -> Result<Node<Data>, Error<'a, Other>> {
        Ok(Node {
            start_token: start,
            end_token: self.token_offset,
            data
        })
    }

    pub fn expect_tokens<Other: Debug + PartialEq>(&mut self, tokens: &[Kind<'a>]) -> Result<Token<'a>, Error<'a, Other>> {
        let peeked = self.tokens
            .peek()
            .ok_or(Error {
                start_token: self.token_offset,
                kind: ErrorKind::ReachedEndForTokens { tokens: Box::from(tokens) },
            })?;

        for &token in tokens { if peeked.kind() == &token {
            let matched_token = self.next().unwrap();
            return Ok(matched_token);
        }}

        let kind = ErrorKind::UnexpectedTokens { expectation: Box::from(tokens), received: *peeked.kind() };
        Err(self.new_error(kind))
    }

    pub fn expect_token<Other: Debug + PartialEq>(&mut self, token: &Kind<'a>) -> Result<Token<'a>, Error<'a, Other>> {
        let peeked = self.tokens
            .peek()
            .ok_or(Error {
                start_token: self.token_offset,
                kind: ErrorKind::ReachedEndForToken { token: *token },
            })?;
        
        if peeked.kind() == token {
            let matched_token = self.next().unwrap();
            return Ok(matched_token);
        }

        let kind = ErrorKind::UnexpectedToken { expectation: *token, received: *peeked.kind() };
        Err(self.new_error(kind))
    }

    pub fn as_restorable<T, E>(&mut self, mut process: impl FnMut(&mut Self) -> Result<T, E>) -> Result<T, E> {
        let restored = self.clone();
        match process(self) {
            Ok(value) => Ok(value),
            Err(error) => {
                *self = restored;
                Err(error)
            }
        }
    }

    pub fn next_identifier<Other: Debug + PartialEq>(&mut self) -> Result<&'a str, Error<'a, Other>> {
        let Some(peeked) = self.peek() else { return Err(self.new_error(ErrorKind::ReachedIdentifierEnd)) };
        if let Kind::Identifier(identifier) = *peeked.kind() {
            let  _ = self.next().unwrap();
            return Ok(identifier);
        }
        
        let copied_peek = *peeked.kind();
        Err(self.new_error(ErrorKind::ExpectedIdentifier { received: copied_peek }))
    }

    // pub fn next_digit<Other: Debug + PartialEq>(&mut self) -> Result<u8, Error<Other>> {
    //     if let Token::Digit(peeked) = *self.tokens.peek()? {
    //         self.next()?;
    //         return Some(peeked)
    //     }
    //     None
    // }

    pub fn peek(&mut self) -> Option<&Token<'a>> {
        self.tokens.peek()
    }
}

impl<'a> From<&'a str> for Traverser<'a> {
    fn from(value: &'a str) -> Self {
        let tokens = token::Iterator::from(value);
        let source = tokens.source();
        
        Self {
            source,
            tokens: tokens.peekable(),
            token_offset: 0,
            string_byte_offset: 0
        }
    }
}

impl<'a> Iterator for Traverser<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.tokens.next()?;
        self.token_offset += 1;
        self.string_byte_offset = result.byte_length();
        Some(result)
    }
}

pub trait Parsable<'a>: Sized {
    type Error: Debug + PartialEq;
    fn parse(tokens: &mut Traverser<'a>) -> Result<Self, node::Error<'a, Self::Error>>;
    fn nodes(&self) -> Option<Vec<NodeVariant>>;
}