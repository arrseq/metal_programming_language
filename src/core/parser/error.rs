use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub struct Error<Kind> {
    kind: Kind,
    position: usize
}