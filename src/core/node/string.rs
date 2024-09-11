use crate::core::node;
use crate::core::node::{Parsable, Traverser};
use crate::core::token::Token;

pub type Node<'a> = node::Node<&'a str>;

impl<'a> Parsable<'a> for Node<'a> {
    type Error = ();

    fn parse(traverser: &mut Traverser<'a>) -> Result<Self, node::Error<'a, Self::Error>> {
        loop {
            traverser.expect_tokens(&[Token::StringQuote, Token::Space, Token::Identifier("hello"), Token::Identifier("world")])?;
        }
        
        todo!()
    }
}