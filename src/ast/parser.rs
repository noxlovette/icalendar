use super::token::Token;
use crate::{
    Calendar,
    ast::token::TokenType::{self, *},
    properties::{
        CalendarScale, Iana, Method, ProductIdentifier, Version, Xprop,
    },
};
use TokenType::*;
use std::str::Utf8Error;
use thiserror::Error;

/// parses Tokens into valid iCal Formal Grammar
#[derive(Default, Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    calendar: CalendarBuilder,
    current: usize,
    depth: Depth,
}

#[derive(Default, Debug)]
struct CalendarBuilder {
    prodid: Option<ProductIdentifier>,
    version: Option<Version>,
    calscale: Option<CalendarScale>,
    method: Option<Method>,
    xprop: Vec<Xprop>,
    iana: Vec<Iana>,
    components: Vec<ComponentBuilder>,
}

impl CalendarBuilder {
    /// creates a new cal builder
    fn new() -> Self {
        Self::default()
    }

    fn build(self) -> Result<Calendar, CalendarError> {
        todo!()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CalendarError {
    #[error("Missing field: {0}")]
    MissingField(&'static str),
}

#[derive(Debug)]
struct ComponentBuilder;

/// how deep in the tree we are
#[derive(Default, Debug)]
pub enum Depth {
    #[default]
    Root,
    Component,
    Property,
    Value,
    Param,
}

impl Depth {
    fn increase(&mut self) {
        match self {
            Self::Root => *self = Self::Component,
            Self::Component => *self = Self::Property,
            Self::Property => *self = Self::Value,
            Self::Value => *self = Self::Param,
            _ => {}
        }
    }
}

impl Parser {
    /// creates a new parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            ..Default::default()
        }
    }

    fn parse(&mut self) -> ParseResult<Calendar> {
        if self.check(Begin)? {
            self.consume(Colon, "expected : after BEGIN clause")?;
            if self.match_tokens(&[
                VEvent, VAlarm, VFreeBusy, VTimezone, VTodo, VJournal,
            ])? {
                self.component()?;
            } else if self.check(VCalendar)? {
                self.calendar()?;
            }
        } else if self.check(End)? {
            self.consume(Colon, "expected : after END clause")?;
        }

        Ok(todo!("built"))
    }

    fn calendar(&mut self) -> ParseResult<Calendar> {
        let mut cal = CalendarBuilder::new();
        match self.property()? {
            ProdId => {
                cal.prodid = Some(self.next()?.lexeme().try_into()?);
            }
            Version => {
                cal.version = Some(self.next()?.lexeme().try_into()?);
            }
            Method => {
                cal.method = Some(self.next()?.lexeme().try_into()?);
            }
            CalScale => {
                cal.calscale = Some(self.next()?.lexeme().try_into()?);
            }
            _ => {} // TODO: check for x and iana
        }

        // done processing the calendar, going one way deeper
        self.depth.increase();
        let components = self.component();

        Ok(cal.build()?)
    }

    fn component(&mut self) -> ParseResult<()> {
        todo!()
    }

    /// checks if the next token corresponds to one of passed token types
    ///
    /// moves on if matches
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
        Ok(self.peek()?.token_type() == t)
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
                line: t.line(),
                msg,
                lexeme: t.lexeme_as_string(),
            },
            Err(e) => e,
        }
    }

    /// checks whether we are at the end of the tokens list
    fn is_at_end(&self) -> ParseResult<bool> {
        Ok(self.peek()?.token_type() == Eof)
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

    /// shorthand for advance, consume colon, return what the token was
    fn property(&mut self) -> ParseResult<TokenType> {
        let tt = self.peek()?.token_type();
        self.next()?;
        self.consume(Colon, "Expected :")?;

        Ok(tt)
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

    #[error(transparent)]
    Calendar(#[from] CalendarError),
}
