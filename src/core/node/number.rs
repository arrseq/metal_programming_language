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

    fn next_number(tokens: &mut Traverser<'a>) -> Result<u64, node::Error<<Self as Parsable<'a>>::Error>> {
        let mut value = 0u64;
        let mut power = 0u32;

        while let Some(digit) = Self::next_digit(tokens) {
            value += 10u64.pow(power) * digit as u64;
            power.checked_add(1).ok_or(tokens.new_other_error(Error::Overflowing))?;
            power += 1;
        }

        if power == 0 { return Err(tokens.new_other_error(Error::ExpectedWholeNumberComponent)) }
        Ok(value)
    }
}

impl<'a> Parsable<'a> for Node {
    type Error = Error;

    fn parse(tokens: &mut Traverser<'a>) -> Result<Self, node::Error<Self::Error>> {
        let start = tokens.token_offset();
        let is_negative = tokens.skip_token(&token::Kind::Negate).is_some();
        let whole = Self::next_number(tokens)?;
        let is_fractional = tokens.skip_token(&token::Kind::Decimal).is_some();

        if is_fractional {
            todo!()
        }

        if is_negative {
            let negated = -i64::try_from(whole).map_err(|_| tokens.new_other_error(Error::Overflowing))?;
            return tokens.end(start, Number::Signed(negated));
        }

        tokens.end(start, Number::UnSigned(whole))
    }

    fn nodes(&self) -> Option<Vec<NodeVariant>> { None }
}