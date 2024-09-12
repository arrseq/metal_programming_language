use crate::core::node;
use crate::core::node::{NodeVariant, Parsable, Traverser};
use crate::core::token::Kind;

// fixme: Use &'a str and setup tokens to track char indexes.
pub type Node = node::Node<Box<str>>;

impl<'a> Parsable<'a> for Node {
    type Error = ();

    fn parse(traverser: &mut Traverser<'a>) -> Result<Self, node::Error<'a, ()>> {
        let start = traverser.token_offset();
        let _ = traverser.expect_token(&Kind::StringQuote)?;
        let mut accumulator = String::new();
        let mut escaping = false;

        while let Some(peeked) = traverser.peek() {
            match peeked.kind() {
                Kind::Escape => {
                    if escaping { accumulator.push('\\') }
                    escaping = !escaping;
                },
                Kind::StringQuote => {
                    if !escaping { 
                        let _ = traverser.next();
                        break
                    }
                    escaping = false;
                    accumulator.push('"');
                },
                _ => {
                    if escaping { return Err(traverser.new_other_error(())) }
                    accumulator = format!("{accumulator}{}", peeked.kind())
                }
            }
            
            let _ = traverser.next();
        }

        traverser.end(start, accumulator.into_boxed_str())
    }

    fn nodes(&self) -> Option<Vec<NodeVariant>> { None }
}