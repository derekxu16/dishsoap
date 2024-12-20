#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrefixOperator {
    Minus,
    Bang,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfixOperator {
    // TODO(derekxu16): Remove this.
    Equals,
    DoubleEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    BitShiftLeft,
    BitShiftRight,
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Dot,
}
