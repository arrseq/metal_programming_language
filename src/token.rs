use crate::node::keyword;

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Name,
    NameEscape,
    Keyword(keyword::Mode)
}