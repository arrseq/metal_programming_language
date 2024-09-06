use std::fmt::Debug;
use thiserror::Error;
use crate::core::parser::traverser::Traverser;

#[derive(Debug, Error, PartialEq)]
pub struct Error<Kind: Debug + PartialEq> {
    pub kind: Kind,
    pub end: usize
}

impl<Kind: Debug + PartialEq> Error<Kind> {
    pub fn from_traverser(traverser: &Traverser, kind: Kind) -> Self {
        Self {
            kind,
            end: traverser.offset()
        }
    }
}