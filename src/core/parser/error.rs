use thiserror::Error;
use crate::core::parser::traverser::Traverser;

#[derive(Debug, Error, PartialEq)]
pub struct Error<Kind> {
    pub kind: Kind,
    pub position: usize
}

impl<Kind> Error<Kind> {
    pub fn from_traverser(traverser: &Traverser, kind: Kind) -> Self {
        Self {
            kind,
            position: traverser.offset()
        }
    }
}