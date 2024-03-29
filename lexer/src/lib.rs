mod token;

pub use self::token::Token;
pub use logos::Logos;

#[derive(PartialEq, Eq)]
pub struct LexerError {
    pub message: String,
}

pub type LexerResult<'a, T> = Result<T, LexerError>;

pub struct Lexer<'a> {
    logos_lexer: logos::Lexer<'a, Token>,
    peeked_value: Option<Option<Token>>,
    peeked_slice: Option<&'a str>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            logos_lexer: logos::Lexer::new(source),
            peeked_value: None,
            peeked_slice: None,
        }
    }

    pub fn pop(&mut self) -> Option<Token> {
        match self.peeked_value {
            Some(t) => {
                self.peeked_value = None;
                self.peeked_slice = None;
                t
            }
            None => self.logos_lexer.next(),
        }
    }

    pub fn peek(&mut self) -> Option<Token> {
        match self.peeked_value {
            Some(t) => t,
            None => {
                self.peeked_value = Some(self.logos_lexer.next());
                self.peeked_slice = Some(self.logos_lexer.slice());
                self.peeked_value.unwrap()
            }
        }
    }

    pub fn slice(&self) -> &str {
        match self.peeked_slice {
            Some(s) => s,
            None => self.logos_lexer.slice(),
        }
    }

    pub fn consume(&mut self, token: Token) -> LexerResult<Token> {
        let consumed: Option<Token> = self.pop();
        if consumed != Some(token) {
            Err(LexerError {
                message: format!(
                    "Compilation error: expected {:?}, but found {:?}",
                    token, consumed
                )
                .to_owned(),
            })
        } else {
            Ok(consumed.unwrap())
        }
    }
}
