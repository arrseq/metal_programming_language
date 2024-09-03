use std::fmt::Debug;
use crate::core::parser::traverser::Traverser;

pub mod whitespace;
pub mod string;

pub trait NodeTrait: Debug + Clone + PartialEq {
    type Error;
    fn parse(traverser: &mut Traverser) -> Result<Self, Self::Error>;
    
    fn get_start(&self) -> usize;
    fn get_end(&self) -> usize;
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
