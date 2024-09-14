use std::fmt::Debug;
use std::ops::{Add, Neg};
use thiserror::Error;
use crate::core::{node, token};
use crate::core::node::{identifier, number, ErrorKind, NodeVariant, Parsable, Traverser};
use crate::core::token::Kind;
use crate::Pbt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scale {
    X8,
    X16,
    X32,
    X64
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumericKind {
    Integer,
    Unsigned,
    Float
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Numeric {
    pub scale: Scale,
    pub kind: NumericKind
}

#[derive(Debug, Clone, PartialEq)]
pub enum MainType<'a> {
    Numeric(Numeric),
    Bool,
    String,
    Other(identifier::Node<'a>)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Modifier {
    Reference,
    MutableReference
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type<'a> {
    main: MainType<'a>,
    modifiers: Box<[Modifier]>
}

pub type Node<'a> = node::Node<MainType<'a>>;

impl<'a> Node<'a> {
    pub const KEYWORDS: [Kind<'a>; 14] = [
        Kind::Identifier("integer1"),
        Kind::Identifier("integer2"),
        Kind::Identifier("integer4"),
        Kind::Identifier("integer8"),
        Kind::Identifier("unsigned1"),
        Kind::Identifier("unsigned2"),
        Kind::Identifier("unsigned4"),
        Kind::Identifier("unsigned8"),
        Kind::Identifier("float1"),
        Kind::Identifier("float2"),
        Kind::Identifier("float4"),
        Kind::Identifier("float8"),
        Kind::Identifier("logical"),
        Kind::Identifier("string")
    ];
    
    pub const MODIFIERS: [Kind<'a>; 2] = [
        Kind::Identifier("ref"),
        Kind::Identifier("mut_ref")
    ];

    fn next_main_type(tokens: &mut Traverser<'a>) -> Result<MainType<'a>, node::Error<<Self as Parsable<'a>>::Error>> {
        let keyword_identifier = identifier::Node::parse(tokens).map_err(|error| tokens.new_other_error(error))?;
        
        // todo: support generics
        Ok(match keyword_identifier.data.0 {
            "integer1"  => MainType::Numeric(Numeric { kind: NumericKind::Integer,  scale: Scale::X8  }),
            "integer2"  => MainType::Numeric(Numeric { kind: NumericKind::Integer,  scale: Scale::X16 }),
            "integer4"  => MainType::Numeric(Numeric { kind: NumericKind::Integer,  scale: Scale::X32 }),
            "integer8"  => MainType::Numeric(Numeric { kind: NumericKind::Integer,  scale: Scale::X64 }),
            "unsigned1" => MainType::Numeric(Numeric { kind: NumericKind::Unsigned, scale: Scale::X8  }),
            "unsigned2" => MainType::Numeric(Numeric { kind: NumericKind::Unsigned, scale: Scale::X16 }),
            "unsigned4" => MainType::Numeric(Numeric { kind: NumericKind::Unsigned, scale: Scale::X32 }),
            "unsigned8" => MainType::Numeric(Numeric { kind: NumericKind::Unsigned, scale: Scale::X64 }),
            "float1"    => MainType::Numeric(Numeric { kind: NumericKind::Float,    scale: Scale::X8  }),
            "float2"    => MainType::Numeric(Numeric { kind: NumericKind::Float,    scale: Scale::X16 }),
            "float4"    => MainType::Numeric(Numeric { kind: NumericKind::Float,    scale: Scale::X32 }),
            "float8"    => MainType::Numeric(Numeric { kind: NumericKind::Float,    scale: Scale::X64 }),
            "logical"   => MainType::Bool,
            "string"    => MainType::String,
            _ => MainType::Other(identifier::Node::parse(tokens).map_err(|error| tokens.new_other_error(error))?)
        })
    }
    
    fn next_modifier(tokens: &mut Traverser<'a>) -> Result<Modifier, node::Error<<Self as Parsable<'a>>::Error>> {
        Ok(match tokens.expect_tokens(&Self::MODIFIERS)?.kind() {
            Kind::Identifier("ref") => Modifier::Reference,
            Kind::Identifier("mut_ref") => Modifier::MutableReference,
            _ => unreachable!()
        })
    }
}

impl<'a> Parsable<'a> for Node<'a> {
    type Error = node::Error<identifier::Error>;

    fn parse(tokens: &mut Traverser<'a>) -> Result<Self, node::Error<Self::Error>> {
        let start = tokens.token_offset();
        let main = Self::next_main_type(tokens)?;
        tokens.end(start, main)
    }
}