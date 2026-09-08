use super::node::Node;
use super::token::Token;
use crate::{
    ast::token::TokenType::{self, *},
    components::Component::Todo,
};
use TokenType::*;
use std::str::Utf8Error;
use thiserror::Error;

/// parses Tokens into valid iCal Formal Grammar
#[derive(Default, Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    nodes: Vec<Node>,
    current: usize,
}

impl Parser {
    /// creates a new parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            ..Default::default()
        }
    }

    fn begin(&mut self) -> ParseResult<()> {
        if self.check(Begin)? {
            self.consume(Colon, "expected : after BEGIN clause")?;
            if self.match_tokens(&[
                VEvent, VAlarm, VFreeBusy, VTimezone, VTodo, VJournal,
            ])? {
                self.component()?;
            } else if self.check(VCalendar)? {
                self.calendar()?;
            }
        }

        Ok(())
    }

    /// checks for end at the end
    fn calendar(&mut self) -> ParseResult<()> {
        while self.match_tokens(&[ProdId, Version, CalScale, Method])? {
            self.consume(Colon, "Expected : after calendar props")?;
            todo!()
        }
        Ok(())
    }

    /// checks for end at the end
    fn component(&mut self) -> ParseResult<()> {
        todo!()
    }

    /// checks if the next token corresponds to one of passed token types
    fn match_tokens(&mut self, types: &[TokenType]) -> ParseResult<bool> {
        for t in types {
            if self.check(*t)? {
                self.next()?;
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
        &mut self,
        tt: TokenType,
        msg: &'static str,
    ) -> ParseResult<&Token> {
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
    fn peek(&self) -> ParseResult<&Token> {
        self.tokens
            .get(self.current)
            .ok_or(ParseError::UnexpectedEof)
    }

    /// advances the parser and returns the next token
    fn next(&mut self) -> ParseResult<&Token> {
        if !self.is_at_end()? {
            self.current += 1;
        }
        self.prev()
    }

    /// gets the latest token
    fn prev(&self) -> ParseResult<&Token> {
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
