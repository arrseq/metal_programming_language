use std::iter::Peekable;
use crate::core::lexer::{Token, Tokens};

#[derive(Debug, Clone)]
pub struct Traverser<'a> {
    offset: usize,
    tokens: Peekable<Tokens<'a>>
}

impl<'a> Traverser<'a> {
    pub const fn get_offset(&self) -> usize { self.offset }
    pub fn peek(&mut self) -> Option<&Token> { self.tokens.peek() }
    
    /// Test to see if the next token in the stream is the provided token.
    pub fn test_token(&mut self, token: &Token) -> bool {
        let Some(peeked) = self.peek() else { return false };
        peeked == token
    }
    
    pub fn consume_token(&mut self, token: &Token) -> bool {
        let Some(peeked) = self.peek() else { return false };
        if peeked != token { return false }
        self.next().unwrap();
        true
    }
}

impl<'a> From<&'a str> for Traverser<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            offset: 0,
            tokens: Tokens::from(value).peekable()
        }
    }
}

impl<'a> Iterator for Traverser<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tokens.next() {
            None => None,
            Some(token) => {
                self.offset += 1;
                Some(token)
            }
        }
    }
}