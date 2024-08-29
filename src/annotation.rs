#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Keyword,
    Operator,
    Variable,

    Comment,
    String,
    Number,

    Type,
    PrimitiveType
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub start_index: usize,
    pub end_index: usize,
    pub kind: Kind,
    pub error: Option<String>,
    pub warning: Option<String>
}

impl Annotation {
    pub const fn from_kind(start: usize, end: usize, kind: Kind) -> Self {
        Self {
            start_index: start,
            end_index: end,
            kind,
            error: None,
            warning: None
        }
    }
}