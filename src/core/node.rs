pub mod string;

use std::cell::RefCell;
use std::fmt::Debug;
use std::iter::Peekable;
use thiserror::Error;
use crate::core::{node, token};
use crate::core::token::Token;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeKind {
    String
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
    ReachedEndForTokens { tokens: Box<[Token<'a>]> },
    #[error("Reached the end of the parsable stream when {nodes:?} was expected")]
    ReachedEndForNodes { nodes: Box<[NodeKind]> },
    #[error("Expected token {expectation:?} but received {received} instead")]
    UnexpectedToken { expectation: Box<[Token<'a>]>, received: Token<'a> },
    #[error("Expected node {expectation:?} but received {received:?} instead")]
    UnexpectedNode { expectation: Box<[NodeKind]>, received: NodeKind },
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
    offset: usize
}

impl<'a> Traverser<'a> {
    pub fn expect_tokens<Other: Debug + PartialEq>(&mut self, tokens: &[Token<'a>]) -> Result<Token<'a>, Error<'a, Other>> {
        let peeked = self.tokens
            .peek()
            .ok_or(Error {
                start_token: self.offset,
                kind: ErrorKind::ReachedEndForTokens { tokens: Box::from(tokens) },
            })?;

        for &token in tokens { if peeked == &token {
            let matched_token = self.next().unwrap();
            return Ok(matched_token);
        }}

        Err(Error {
            start_token: self.offset,
            kind: ErrorKind::UnexpectedToken { expectation: Box::from(tokens), received: *peeked },
        })
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
}

impl<'a> From<&'a str> for Traverser<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            tokens: token::Iterator::from(value).peekable(),
            offset: 0
        }
    }
}

impl<'a> Iterator for Traverser<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.tokens.next()?;
        self.offset += 1;
        Some(result)
    }
}

pub trait Parsable<'a>: Sized {
    type Error: Debug + PartialEq;
    fn parse(tokens: &mut Traverser<'a>) -> Result<Self, node::Error<'a, Self::Error>>;
}