use std::iter::{Enumerate, Peekable};
use std::str::Chars;
use crate::{annotation, comment};
use crate::annotation::Annotation;
use crate::operator::Operator;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    iterator: Peekable<Enumerate<Chars<'a>>>,
    pub annotations: Vec<Annotation>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub keyword: String,
    pub name: String,
    // todo: Add generics
}

impl<'a> Parser<'a> {
    pub const fn new(chars: Peekable<Enumerate<Chars<'a>>>) -> Self {
        Self { iterator: chars, annotations: Vec::new() }
    }

    pub fn get_current(&mut self) -> Option<usize> {
        self.iterator.peek().map(|(pos, _)| *pos).map(|pos| pos.saturating_sub(1))
    }

    pub fn parse_identifier(&mut self) -> Option<String> {
        let mut identifier = String::new();

        while let Some((_, c)) = self.iterator.peek() {
            if c.is_alphanumeric() || c == &'_' {
                identifier.push(*c);
                self.iterator.next().unwrap();
            }
            else { break }
        }

        if identifier.is_empty() { None }
        else { Some(identifier) }
    }

    pub fn parse_symbol(&mut self) -> Option<String> {
        let mut symbol = String::new();

        while let Some((_, c)) = self.iterator.peek() {
            if !c.is_alphanumeric() && c != &'_' {
                symbol.push(*c);
                self.iterator.next();
            } else { break }
        }

        if symbol.is_empty() { None }
        else { Some(symbol) }
    }

    pub fn parse_name(&mut self) -> Option<String> {
        let start = self.get_current()?;
        let ident = self.parse_identifier()?;
        let end = self.get_current()?;

        self.annotations.push(Annotation::from_kind(start, end, annotation::Kind::Variable));
        Some(ident)
    }

    pub fn parse_repeating_sequence(&mut self, symbol: char) -> Option<usize> {
        let mut count = 0;

        while let Some(&c) = self.iterator.peek() {
            if c.1 == symbol {
                count += 1;
                self.iterator.next();
            }
            else { break; }
        }

        if count > 0 { Some(count) }
        else { None }
    }

    pub fn parse_char(&mut self, symbol: char) -> Option<()> {
        if let Some(&c) = self.iterator.peek() {
            if c.1 == symbol {
                self.iterator.next();
                Some(())
            }
            else { None }
        }
        else { None }
    }

    /// Parses misc characters that do not influence the grammar such as ' ','\n', and '\t'.
    pub fn parse_empty(&mut self) {
        while let Some(&c) = self.iterator.peek() {
            if c.1 == ' ' || c.1 == '\n' || c.1 == '\t' { self.iterator.next(); }
            else { break }
        }
    }

    /// Parse a keyword declaration which involves a keyword ad then a name.
    pub fn parse_keyword_header(&mut self) -> Option<Declaration> {
        let kw_start = self.get_current()?;
        let keyword = self.parse_identifier()?;
        let kw_end = self.get_current()?;
        self.annotations.push(Annotation::from_kind(kw_start, kw_end, annotation::Kind::Keyword));

        self.parse_empty();

        let name = self.parse_name()?;
        Some(Declaration { keyword, name })
    }

    pub fn parse_integer(&mut self) -> Option<usize> {
        // read first digit to know if this is a number or not.
        let mut output = if let Some((_, ch)) = self.iterator.peek() {
            let digit = ch.to_digit(10)? as usize;
            self.iterator.next().unwrap();
            digit
        } else { return None };

        // This code works differently because we read the number left to right.
        while let Some((_, ch)) = self.iterator.peek() {
            let Some(digit) = ch.to_digit(10) else { return Some(output) };
            self.iterator.next().unwrap();

            // increase place value.
            output *= 10;
            // offset current digit.
            output += digit as usize;
        }

        Some(output)
    }

    pub fn parse_float(&mut self) -> Option<f64> {
        let whole = self.parse_integer()?;
        let mut output = whole as f64;
        let Some(_) = self.parse_char('.') else { return Some(output) };

        // parse decimal
        let mut fractional_place = 1i32; // start at 1 not 0 so that 0._ is the first value.

        while let Some((_, ch)) = self.iterator.peek() {
            let Some(digit) = ch.to_digit(10) else { return Some(output) };
            self.iterator.next().unwrap();

            output += digit as f64 / 10f64.powi(fractional_place);
            dbg!(digit as f64 / 10f64.powi(fractional_place));
            fractional_place += 1;
        }

        Some(output)
    }

    pub fn parse_bool(&mut self) -> Option<bool> {
        Some(match self.parse_identifier()?.to_lowercase().as_str() {
            "true" => true,
            "false" => false,
            _ => return None
        })
    }

    pub fn parse_operator(&mut self) -> Option<Operator> {
        let symbol = self.parse_symbol()?;
        Operator::from_str(symbol.as_str())
    }

    pub fn parse_line_comment(&mut self) -> Option<String> {
        for _ in 0..2 { self.parse_char('/')? }
        self.parse_char(' ')?;
        
        todo!()
    }
    
    pub fn parse_comment_notation(&mut self) -> Option<(String, Vec<comment::Annotation>> {
        
    }

    pub fn parse_until(&mut self, stop: char) -> Option<String> {
        let mut result = String::new();
        let mut escaped = false;

        while let Some((_, c)) = self.iterator.peek() {
            if c == &stop && !escaped { break }
            else if c == &'\\' {
                escaped = !escaped;
                if !escaped { result.push(*c) }
            } else { result.push(*c) }

            self.iterator.next().unwrap();
        }

        if result.is_empty() { None } else { Some(result) }
    }
}