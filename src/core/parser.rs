use thiserror::Error;
use crate::core::lexer;

pub mod node;
pub mod traverser;
mod error;

#[derive(Debug, Error)]
#[error("Expected one of the following tokens")]
pub struct Error(Vec<lexer::Token>);