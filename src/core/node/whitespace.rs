use thiserror::Error;
use crate::core::node;
use crate::core::node::{NodeVariant, Parsable, Traverser};
use crate::core::token::Kind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WhiteSpace {
    Space,
    Tab,
    NewLine
}

pub type Node = node::Node<Box<[WhiteSpace]>>;

impl<'a> Node {
    pub const WHITESPACE_TOKENS: [Kind<'a>; 3] = [Kind::Space, Kind::Tab, Kind::NewLine];
}

impl<'a> Parsable<'a> for Node {
    type Error = ();

    fn parse(traverser: &mut Traverser<'a>) -> Result<Self, node::Error<'a, Self::Error>> {
        let start = traverser.token_offset();
        let mut accumulator = Vec::new();
        loop {
            let Ok(token) = traverser.expect_tokens::<()>(&Self::WHITESPACE_TOKENS) else { break };
            let whitespace = match token.kind() {
                Kind::Space => WhiteSpace::Space,
                Kind::Tab => WhiteSpace::Tab,
                Kind::NewLine => WhiteSpace::NewLine,
                _ => unreachable!()
            };
            accumulator.push(whitespace);
        }
        
        if accumulator.is_empty() { return Err(traverser.new_other_error(())) }
        traverser.end(start, accumulator.into_boxed_slice())
    }

    fn nodes(&self) -> Option<Vec<NodeVariant>> { None }
}