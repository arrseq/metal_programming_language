use std::fmt::Debug;
use thiserror::Error;
use crate::core::{node, token};
use crate::core::node::{ErrorKind, NodeVariant, Parsable, Traverser};
use crate::core::token::Kind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    UnSigned(u64),
    Signed(i64),
    Float(f64)
}

pub type Node = node::Node<Number>;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("The number is too large to be stored as a literal")]
    Overflowing,
    #[error("Expected whole number")]
    ExpectedWholeNumberComponent,
    #[error("Expected fractional number after decimal separator")]
    ExpectedFractionalNumberComponent
}

impl<'a> Node {
    fn next_digit(tokens: &mut Traverser<'a>) -> Option<u8> {
        let peeked = tokens.peek()?;
        if let Kind::Digit(digit) = *peeked.kind() {
            let  _ = tokens.next()?;
            return Some(digit);
        }

        None
    }

    fn next_number<Other: Debug + PartialEq>(tokens: &mut Traverser<'a>) -> Result<u64, node::Error<Other>> {
        let mut value = 0;
        let mut power = 0;
        
        while let Ok(digit) = Self::next_digit::<Other>(tokens) {
            
        }
        
        if power == 0 {  }
        
        todo!()
    }
}

impl<'a> Parsable<'a> for Node {
    type Error = Error;

    fn parse(tokens: &mut Traverser<'a>) -> Result<Self, node::Error<Self::Error>> {
        let start = tokens.token_offset();
        let is_negative = tokens.skip_token(&token::Kind::Negate).is_some();
        
        todo!()
    }

    fn nodes(&self) -> Option<Vec<NodeVariant>> { None }
}