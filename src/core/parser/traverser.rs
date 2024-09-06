use std::iter::Peekable;
use crate::core::lexer::{Token, Tokens};

#[derive(Debug, Clone)]
pub struct Traverser<'a> {
    offset: usize,
    tokens: Peekable<Tokens<'a>>
}

impl<'a> Traverser<'a> {
    pub const fn offset(&self) -> usize { self.offset }
    pub fn peek(&mut self) -> Option<&Token> { self.tokens.peek() }
    
    pub fn try_consume_token(&mut self, token: &Token) -> bool {
        let Some(peeked) = self.peek() else { return false };
        if peeked != token { return false }
        self.next().unwrap();
        true
    }
    
    pub fn test_token<Output>(&mut self, mut test: impl FnMut(&Token) -> Option<Output>) -> Option<Output> {
        let Some(peeked) = self.peek() else { return None };
        if let Some(capture) = test(peeked) { 
            let _ = self.next();
            Some(capture)
        }
        else { None }
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