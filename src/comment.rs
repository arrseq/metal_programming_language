#[derive(Debug, Clone, PartialEq)]
pub struct TypeReference {
    pub start: usize,
    pub end: usize,
    pub parts: Vec<String>
}