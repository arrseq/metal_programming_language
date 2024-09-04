use std::fmt::Debug;
use crate::core::parser::traverser::Traverser;

pub mod whitespace;
pub mod string;

pub trait NodeTrait: Debug + Clone + PartialEq {
    type Error;
    fn parse(traverser: &mut Traverser) -> Result<Self, Self::Error>;
    
    /// Get the start token index of this node.
    fn start(&self) -> usize;
    /// Get the end token index of this node.
    fn end(&self) -> usize;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    WhiteSpace
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    kind: Kind,
    // The index of the token where this node starts.
    start: usize,
    // The index of the last token that built this node.
    end: usize
}
