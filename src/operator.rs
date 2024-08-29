#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    And,
    Or,
    Equal,
    ShiftLeft,
    ShiftRight,
    GreaterThan,
    LessThan
}

impl Operator {
    pub fn from_str(ch: &str) -> Option<Self> {
        Some(match ch {
            "+"  => Operator::Add,
            "-"  => Operator::Subtract,
            "*"  => Operator::Multiply,
            "/"  => Operator::Divide,
            "&"  => Operator::And,
            "|"  => Operator::Or,
            "="  => Operator::Equal,
            "<<" => Operator::ShiftLeft,
            ">>" => Operator::ShiftRight,
            ">"  => Operator::GreaterThan,
            "<"  => Operator::LessThan,
            _ => return None
        })
    }
}