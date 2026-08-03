use super::token::Token;
use crate::ast::token::TokenType::{self, *};
use std::str::Utf8Error;
use thiserror::Error;

/// parses Tokens into valid iCal Formal Grammar
#[derive(Default, Debug)]
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    /// creates a new parser
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            ..Default::default()
        }
    }

    /// checks if the next token corresponds to one of passed token types
    fn match_tokens(&'a mut self, types: &'a [TokenType]) -> ParseResult<bool> {
        for t in types {
            if self.check(*t)? {
                self.next();
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// returns true if the next token corresponds to the one passed to the function. false if we have reached the end of the vector
    fn check(&mut self, t: TokenType) -> ParseResult<bool> {
        Ok(self.peek()?.get_type() == t)
    }

    /// consumes a token that a certain grammar rule expects
    fn consume(
        &'a mut self,
        tt: TokenType,
        msg: &'static str,
    ) -> ParseResult<&'a Token<'a>> {
        if self.check(tt)? {
            self.next()
        } else {
            Err(self.error(msg))
        }
    }

    /// error convenience wrapper
    fn error(&self, msg: &'static str) -> ParseError {
        match self.peek() {
            Ok(t) => ParseError::UnexpectedToken {
                line: t.get_line(),
                msg,
                lexeme: t.get_lexeme_as_string(),
            },
            Err(e) => e,
        }
    }

    /// checks whether we are at the end of the tokens list
    fn is_at_end(&self) -> ParseResult<bool> {
        Ok(self.peek()?.get_type() == Eof)
    }

    /// returns the next token. doesn't advance the parser
    fn peek(&'a self) -> ParseResult<&'a Token<'a>> {
        self.tokens
            .get(self.current)
            .ok_or(ParseError::UnexpectedEof)
    }

    /// advances the parser and returns the next token
    fn next(&'a mut self) -> ParseResult<&'a Token<'a>> {
        if !self.is_at_end()? {
            self.current += 1;
        }
        self.prev()
    }

    /// gets the latest token
    fn prev(&'a self) -> ParseResult<&'a Token<'a>> {
        self.tokens
            .get(self.current - 1)
            .ok_or(ParseError::EmptyTokenList)
    }
}

/// Convenience wrapper for [ParseError]
pub(crate) type ParseResult<T> = Result<T, ParseError>;

#[derive(Error, Debug)]
/// Parsing error
pub enum ParseError {
    /// Parameter Parsing Error
    #[error("Parameter parsing failed. Expected {expected}, got {received:?}")]
    Parameter {
        /// What the parameter is supposed to be
        expected: String,
        /// What we actually received
        received: Option<String>,
    },

    /// URL parsing error
    #[error("Incorrect URL: {0}")]
    URL(#[from] url::ParseError),

    /// Quoted String Error
    #[error("Not a quoted string value")]
    QuotedString,

    /// Encoding error
    #[error("UTF-8 Error")]
    UTF(#[from] Utf8Error),

    /// [CalendarUserAddress] Parsing Error
    #[error("Malformed CalenderUserAddress")]
    CalUserAddress,

    /// [MediaType] Parsing Error
    #[error("Malformed MediaType")]
    MediaType,

    /// [Language] Parsing Error
    #[error("Malformed Language")]
    Language,

    #[error("Malformed Boolean")]
    Boolean,

    #[error("Unexpected EOF")]
    UnexpectedEof,

    #[error("Unexpected Token at line {line} ({lexeme}): {msg}")]
    UnexpectedToken {
        line: usize,
        msg: &'static str,
        lexeme: String,
    },

    #[error("Ran prev on an empty token list")]
    EmptyTokenList,
}
