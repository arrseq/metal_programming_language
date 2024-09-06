#[cfg(test)]
mod test;

use crate::core::lexer::Token;
use crate::core::parser::node;
use crate::core::parser::node::error;
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
    symbols: Box<[Symbol]>
}

impl node::Node for Node {
    type Error = ();

    fn parse(traverser: &mut Traverser) -> Result<Self, error::Error<Self::Error>> {
        let start = traverser.offset();
        let mut symbols = Vec::new();
        while let Some(token) = traverser.peek() {
            let Ok(symbol) = Symbol::try_from(token) else { break };
            symbols.push(symbol);
            traverser.next().unwrap();
        }
        
        if symbols.is_empty() { return Err(error::Error::from_traverser(&traverser, ())) }
        
        Ok(Self {
            start,
            end: traverser.offset(),
            symbols: Box::<[Symbol]>::from(symbols)
        })
    }

    fn start(&self) -> usize { self.start }
    fn end(&self) -> usize { self.end }
}