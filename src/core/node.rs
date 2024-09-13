pub mod identifier;
pub mod string;
pub mod whitespace;
pub mod number;

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
    Identifier,
    Number
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeVariant<'a> {
    WhiteSpace(whitespace::Node),
    String(string::Node<'a>),
    Identifier(identifier::Node<'a>),
    Number(number::Node)
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
pub enum ErrorKind<Other: Debug + PartialEq> {
    #[error("Reached the end of stream when attempting to match a token")]
    ReachedEndForToken,
    #[error("Reached the end of stream when attempting to match a node")]
    ReachedEndForNode,
    #[error("An unexpected token was encountered")]
    UnexpectedToken,
    #[error("Failed to parse node due to other reason")]
    Other(Other)
}

#[derive(Debug, Error, PartialEq)]
#[error("Failed to parse a node")]
pub struct Error<Other: Debug + PartialEq> {
    pub kind: ErrorKind<Other>,
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

    pub const fn new_error<Other: Debug + PartialEq>(&self, kind: ErrorKind<Other>) -> Error<Other> {
        Error {
            kind,
            start_token: self.token_offset,
        }
    }

    pub const fn new_other_error<Other: Debug + PartialEq>(&self, other: Other) -> Error<Other> {
        self.new_error(ErrorKind::Other(other))
    }

    pub const fn end<Other: Debug + PartialEq, Data: Debug + Clone + PartialEq>(&self, start: usize, data: Data) -> Result<Node<Data>, Error<Other>> {
        Ok(Node {
            start_token: start,
            end_token: self.token_offset,
            data
        })
    }

    pub fn skip_tokens(&mut self, tokens: &[Kind<'a>]) -> Option<Token> {
        let peeked = self.peek()?;
        if !tokens.iter().any(|x| x == peeked.kind()) { return None }
        Some(*peeked)
    }

    pub fn skip_token(&mut self, token: &Kind<'a>) -> Option<Token> {
        let peeked = self.peek()?;
        if token != peeked.kind() { return None }
        self.next()
    }

    pub fn expect_tokens<Other: Debug + PartialEq>(&mut self, tokens: &[Kind<'a>]) -> Result<Token<'a>, Error<Other>> {
        let peeked = self.tokens
            .peek()
            .ok_or(Error {
                start_token: self.token_offset,
                kind: ErrorKind::ReachedEndForToken,
            })?;

        for &token in tokens { if peeked.kind() == &token {
            let matched_token = self.next().unwrap();
            return Ok(matched_token);
        }}

        Err(self.new_error(ErrorKind::UnexpectedToken))
    }

    pub fn expect_token<Other: Debug + PartialEq>(&mut self, token: &Kind<'a>) -> Result<Token<'a>, Error<Other>> {
        let peeked = self.tokens
            .peek()
            .ok_or(Error {
                start_token: self.token_offset,
                kind: ErrorKind::ReachedEndForToken,
            })?;
        
        if peeked.kind() == token {
            let matched_token = self.next().unwrap();
            return Ok(matched_token);
        }

        Err(self.new_error(ErrorKind::UnexpectedToken))
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
    fn parse(tokens: &mut Traverser<'a>) -> Result<Self, node::Error<Self::Error>>;
    fn nodes(&self) -> Option<Vec<NodeVariant>>;
}