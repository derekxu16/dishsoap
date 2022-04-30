#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrefixOperator {
    Minus,
    Bang,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfixOperator {
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    BitShiftLeft,
    BitShiftRight,
    DoubleEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Equals,
}
