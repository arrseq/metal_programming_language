#[cfg(test)]
mod test;

use std::iter::Peekable;
use crate::core::lexer::{Token, Tokens};
use crate::core::parser::node;
use crate::core::parser::node::{error, Node};
use crate::core::parser::node::error::Error;

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

    pub fn test_token<Output>(&mut self, mut test: impl FnMut(Token) -> Option<Output>) -> Option<Output> {
        // todo: Tis function could use self.use_reverting
        
        let original = self.tokens.clone();
        let Some(token) = self.next() else {
            self.tokens = original;
            return None;
        };

        let Some(output) = test(token) else {
            self.tokens = original;
            return None;
        };

        Some(output)
    }
    
    pub fn use_reverting<Output>(&mut self, mut operation: impl FnMut(&mut Self) -> Option<Output>) -> Option<Output> {
        let original = self.tokens.clone();
        let Some(output) = operation(self) else {
            self.tokens = original;
            return None;
        };
        
        Some(output)
    }
    
    pub fn try_parse_node<Type: node::Node>(&mut self) -> Result<Type, error::Error<Type::Error>> {
        let mut error = None;
        let result = self.use_reverting(|traverser| match Type::parse(traverser) {
            Ok(result) => Some(result),
            Err(e) => {
                error = Some(e);
                None
            }
        });
        
        if let Some(error) = error { return Err(error) }
        Ok(result.unwrap())
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