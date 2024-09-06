#[cfg(test)]
mod test;

use thiserror::Error;
use crate::core::lexer::Token;
use crate::core::parser::node;
use crate::core::parser::node::error;
use crate::core::parser::traverser::Traverser;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number {
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    start: usize,
    end: usize,
    value: Number
}

impl Node {
    pub const fn value(&self) -> Number { self.value }
}

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("The whole number component was expected")]
    ExpectedWholeNumber,
    #[error("The fractional component was expected because a decimal point was used")]
    ExpectedFractionalNumber,
    #[error("Found an identifier when a number was expected")]
    FoundIdentifier
}

impl Node {
    /// Read the next number or fail with an error
    fn next_number(traverser: &mut Traverser, error: Error) -> Result<u64, error::Error<<Self as node::Node>::Error>> {
        Ok(traverser
            .test_token(|token| if let Token::Number(number) = token { Some(*number) } else { None })
            .ok_or(error::Error::from_traverser(&traverser, error))?)
    }
    
    fn parse_base_number(traverser: &mut Traverser) -> Result<Number, error::Error<<Self as node::Node>::Error>> {
        todo!()
    }
}


impl Node {
    fn test_identifier(traverser: &mut Traverser) -> Result<(), error::Error<<Self as node::Node>::Error>> {
        if matches!(traverser.peek(), Some(Token::Identifier(_))) { return Err(error::Error::from_traverser(&traverser, Error::FoundIdentifier)) }
        Ok(())
    }
}

impl node::Node for Node {
    type Error = Error;

    fn parse(traverser: &mut Traverser) -> Result<Self, error::Error<Self::Error>> {
        let start = traverser.offset();
        let negate = traverser.try_consume_token(&Token::SubtractOperator);
        let whole = Self::next_number(traverser, Error::ExpectedWholeNumber)?;
        
        // todo: Implement checks for numbers if they overflow in normal memory.
        
        // Floating point number.
        if traverser.try_consume_token(&Token::Comma) {
            let fractional_integer = Self::next_number(traverser, Error::ExpectedFractionalNumber)?;
            let digits_count = (fractional_integer.ilog10() + 1) as i32; // todo: Benchmark performance for kendal algorithm
            let fractional = fractional_integer as f64 / 10f64.powi(digits_count);
            let mut value = whole as f64 + fractional;
            if negate { value = -value }

            Self::test_identifier(traverser)?;
            return Ok(Self {
                start, 
                end: traverser.offset(),
                value: Number::Float(value)
            })
        }
        
        if negate {
            let value = Number::Integer(-(whole as i64));
            Self::test_identifier(traverser)?;
            return Ok(Self {
                start, value,
                end: traverser.offset()
            });
        }

        Self::test_identifier(traverser)?;
        Ok(Self {
            start,
            end: traverser.offset(),
            value: Number::UnsignedInteger(whole)
        })
    }

    fn start(&self) -> usize { self.start }
    fn end(&self) -> usize { self.end }
}