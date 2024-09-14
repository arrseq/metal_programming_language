// use std::fmt::Debug;
// use std::ops::{Add, Neg};
// use thiserror::Error;
// use crate::core::{node, token};
// use crate::core::node::{ErrorKind, NodeVariant, Parsable, Traverser};
// use crate::core::token::Kind;
// 
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct Header {
//     
// }
// 
// pub type Node = node::Node<Header>;
// 
// #[derive(Debug, Error, PartialEq)]
// pub enum Error {
//     #[error("The number is too large to be stored as a literal")]
//     OverflowingWhole,
//     #[error("The fractional component is too large to be stored as a literal")]
//     OverflowingFractional,
//     #[error("Expected whole number")]
//     ExpectedWholeNumberComponent,
//     #[error("Expected fractional number after decimal separator")]
//     ExpectedFractionalNumberComponent
// }
// 
// impl<'a> Node {
//     fn next_digit(tokens: &mut Traverser<'a>) -> Option<u8> {
//         let peeked = tokens.peek()?;
//         if let Kind::Digit(digit) = *peeked.kind() {
//             let  _ = tokens.next()?;
//             return Some(digit);
//         }
// 
//         None
//     }
// 
//     fn next_number(tokens: &mut Traverser<'a>) -> Result<u64, node::Error<<Self as Parsable<'a>>::Error>> {
//         let mut value = 0u64;
//         let mut power = 0u32;
// 
//         while let Some(digit) = Self::next_digit(tokens) {
//             let digit_offset = 10u64
//                 .checked_pow(power)
//                 .ok_or(tokens.new_other_error(Error::OverflowingWhole))?
//                 .checked_mul(digit as u64)
//                 .ok_or(tokens.new_other_error(Error::OverflowingWhole))?;
//             value = value
//                 .checked_add(digit_offset)
//                 .ok_or(tokens.new_other_error(Error::OverflowingWhole))?;
// 
//             power = power
//                 .checked_add(1)
//                 .ok_or(tokens.new_other_error(Error::OverflowingWhole))?;
//         }
// 
//         if power == 0 { return Err(tokens.new_other_error(Error::ExpectedWholeNumberComponent)) }
//         Ok(value)
//     }
// 
//     fn next_decimal(tokens: &mut Traverser<'a>) -> Result<f64, node::Error<<Self as Parsable<'a>>::Error>> {
//         let mut accumulator = 0f64;
//         let mut division = 0i32;
// 
//         while let Some(digit) = Self::next_digit(tokens) {
//             // adding to the divisor is done first so that we can check to see if digits were read
//             // by testing whether the division is not zero.
//             division = division
//                 .checked_add(1)
//                 .ok_or(tokens.new_other_error(Error::OverflowingFractional))?;
// 
//             // fixme: Floats do not overflow, but we need to wash out numbers that are wastefully
//             // fixme: large and are infinity.
//             accumulator = accumulator.add(digit as f64 / 10f64.powi(division));
//         }
// 
//         if division == 0 { return Err(tokens.new_other_error(Error::ExpectedFractionalNumberComponent)) }
//         Ok(accumulator)
//     }
// }
// 
// impl<'a> Parsable<'a> for Node {
//     type Error = Error;
// 
//     fn parse(tokens: &mut Traverser<'a>) -> Result<Self, node::Error<Self::Error>> {
//         let start = tokens.token_offset();
//         let is_negative = tokens.skip_token(&token::Kind::Negate).is_some();
//         let whole = Self::next_number(tokens)?;
//         let is_fractional = tokens.skip_token(&token::Kind::Decimal).is_some();
// 
//         if is_fractional {
//             let mut fractional = Self::next_decimal(tokens)?;
//             fractional += whole as f64; // fixme: check for data inconsistency potential.
//             if is_negative { fractional = fractional.neg() }
//             return tokens.end(start, Number::Float(fractional));
//         }
// 
//         if is_negative {
//             let negated = -i64::try_from(whole).map_err(|_| tokens.new_other_error(Error::OverflowingWhole))?;
//             return tokens.end(start, Number::Signed(negated));
//         }
// 
//         tokens.end(start, Number::UnSigned(whole))
//     }
// 
//     fn nodes(&self) -> Option<Vec<NodeVariant>> { None }
// }