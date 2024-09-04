#[cfg(test)]
mod test;

use crate::core::lexer::{Token, Tokens};
use crate::core::parser::node::NodeTrait;
use crate::core::parser::traverser::Traverser;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Symbol {
    Space,
    Tab,
    NewLine
}

impl TryFrom<&Token> for Symbol {
    type Error = ();

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        Ok(match value {
            Token::Space => Symbol::Space,
            Token::Tab => Symbol::Tab,
            Token::NewLine => Symbol::NewLine,
            _ => return Err(())
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    start: usize,
    end: usize,
    symbols: Vec<Symbol>
}

impl NodeTrait for Node {
    type Error = ();

    fn parse(traverser: &mut Traverser) -> Result<Self, Self::Error> {
        let mut construct = Self { 
            start: traverser.get_offset(),
            end: 0,
            symbols: Vec::new()
        };
        while let Some(token) = traverser.peek() {
            let Ok(symbol) = Symbol::try_from(token) else { break };
            construct.symbols.push(symbol);
            traverser.next().unwrap();
        }
        construct.end += traverser.get_offset();
        
        Ok(construct)
    }

    fn start(&self) -> usize { self.start }
    fn end(&self) -> usize { self.end }
}