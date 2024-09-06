use std::fmt::Debug;
use crate::core::parser::traverser::Traverser;

pub mod whitespace;
pub mod string;
pub mod number;
mod error;

pub trait Node: Sized + Debug {
    type Error: Debug + PartialEq;
    fn parse(traverser: &mut Traverser) -> Result<Self, error::Error<Self::Error>>;
    
    /// Get the start token index of this node.
    fn start(&self) -> usize;
    /// Get the end token index of this node.
    fn end(&self) -> usize;
}