pub struct ParserError {
    pub message: String,
}

pub type ParserResult<'a, T> = Result<T, ParserError>;
