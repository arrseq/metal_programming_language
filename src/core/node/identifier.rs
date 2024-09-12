use crate::core::node;
use crate::core::node::{Error, NodeVariant, Parsable, Traverser};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Identifier<'a>(&'a str);

pub type Node<'a> = node::Node<Identifier<'a>>;

impl<'a> Parsable<'a> for Node<'a> {
    type Error = ();

    fn parse(tokens: &mut Traverser<'a>) -> Result<Self, Error<'a, Self::Error>> {
        let start = tokens.token_offset();
        let identifier = tokens.next_identifier()?;
        tokens.end(start, Identifier(identifier))
    }

    fn nodes(&self) -> Option<Vec<NodeVariant>> { None }
}