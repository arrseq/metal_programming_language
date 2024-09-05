use std::fmt::Debug;
use crate::core::parser::error;
use crate::core::parser::traverser::Traverser;

pub mod whitespace;
pub mod string;
pub mod number;

pub trait Node: Debug + Clone + PartialEq {
    type Error;
    fn parse(traverser: &mut Traverser) -> Result<Self, error::Error<Self::Error>>;
    
    /// Get the start token index of this node.
    fn start(&self) -> usize;
    /// Get the end token index of this node.
    fn end(&self) -> usize;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    WhiteSpace
}