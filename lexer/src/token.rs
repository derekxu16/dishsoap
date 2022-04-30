use logos::Logos;

#[derive(Logos, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    #[token(";")]
    Semicolon,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token(".")]
    Accessor,

    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClose,

    #[token("->")]
    Arrow,

    #[token("=>")]
    FatArrow,

    #[regex("[a-zA-Z_$][a-zA-Z0-9_$]*")]
    Identifier,

    #[token("func")]
    FuncKeyword,

    #[token("let")]
    LetKeyword,

    #[token("const")]
    ConstKeyword,

    #[token("if")]
    IfKeyword,

    #[token("else")]
    ElseKeyword,

    #[token("return")]
    ReturnKeyword,

    // Literals
    #[token("true")]
    TrueKeyword,

    #[token("false")]
    FalseKeyword,

    #[regex("0[xX][0-9a-fA-F]+")]
    HexLiteral,

    #[regex("[0-9]+")]
    IntegerLiteral,

    #[regex("[0-9]*\\.[0-9]+([eE][+-]?[0-9]+)?|[0-9]+[eE][+-]?[0-9]+")]
    RationalLiteral,

    #[regex("\"([^\"\\\\]|\\\\.)*\"")]
    #[regex("'([^'\\\\]|\\\\.)*'")]
    StringLiteral,

    // Types
    #[token("P_unit")]
    UnitPrimitiveKeyword,

    #[token("P_bool")]
    BoolPrimitiveKeyword,

    #[token("P_i32")]
    I32PrimitiveKeyword,

    // Operators
    #[token("++")]
    OperatorIncrement,

    #[token("--")]
    OperatorDecrement,

    #[token("!")]
    Bang,

    #[token("~")]
    OperatorBitNot,

    #[token("*")]
    Times,

    #[token("/")]
    Divide,

    #[token("%")]
    Percent,

    #[token("**")]
    DoubleAsterisk,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("<<")]
    BitShiftLeft,

    #[token(">>")]
    BitShiftRight,

    #[token("<")]
    LessThan,

    #[token("<=")]
    LessThanEquals,

    #[token(">")]
    GreaterThan,

    #[token(">=")]
    GreaterThanEquals,

    #[token("==")]
    DoubleEquals,

    #[token("!=")]
    NotEquals,

    #[token("&")]
    OperatorBitAnd,

    #[token("^")]
    OperatorBitXor,

    #[token("|")]
    OperatorBitOr,

    #[token("&&")]
    OperatorLogicalAnd,

    #[token("||")]
    OperatorLogicalOr,

    #[token("?")]
    OperatorConditional,

    #[token("=")]
    Equals,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(r"//.*[\n\r]", logos::skip)]
    Skip,

    #[error]
    UnexpectedToken,
}
