use crate::core::node;
use crate::core::node::{NodeVariant, Parsable, Traverser};
use crate::core::token::Kind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct String<'a>(&'a str);

pub type Node<'a> = node::Node<String<'a>>;

impl<'a> Parsable<'a> for Node<'a> {
    type Error = ();

    fn parse(traverser: &mut Traverser<'a>) -> Result<Self, node::Error<'a, ()>> {
        let start = traverser.token_offset();
        let _ = traverser.expect_token(&Kind::StringQuote)?;
        let byte_start = traverser.byte_offset();
        let mut byte_end = byte_start;
        let mut escaping = false;

        while let Some(peeked) = traverser.peek() {
            match peeked.kind() {
                Kind::Escape => {
                    if escaping { byte_end += '\\'.len_utf8() }
                    escaping = !escaping;
                },
                Kind::StringQuote => {
                    if !escaping { 
                        let _ = traverser.next();
                        break
                    }
                    escaping = false;
                    byte_end += '"'.len_utf8();
                },
                _ => {
                    if escaping { return Err(traverser.new_other_error(())) }
                    byte_end = peeked.byte_length()
                }
            }
            
            let _ = traverser.next();
        }

        traverser.end(start, String(&traverser.source()[byte_start..byte_end]))
    }

    fn nodes(&self) -> Option<Vec<NodeVariant>> { None }
}