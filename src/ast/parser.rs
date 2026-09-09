use super::token::Token;
use crate::{
    Calendar,
    ast::{
        CalendarBuilder, Component, EventBuilder, FreeBusyBuilder,
        JournalBuilder, TodoBuilder, token::TokenType,
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

impl<'a> Parser {
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

        // while we are not at the beginning of the first component
        while !self.check(Begin)? {
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

            self.consume(Crlf, "expected crlf after property")?;
        }
        let mut components = self.component()?;

        todo!("extend");

        // bigger
        self.consume(End, "expected the calendar to have an END")?;

        Ok(cal.build()?)
    }

    /// recursive function that returns a vec of components for a calendar
    fn component(&mut self) -> ParseResult<Vec<Component>> {
        self.consume(Begin, "Expected component to begin with BEGIN")?;
        self.consume(Colon, "expected : after BEGIN clause")?;
        let c: Component = match self.next()?.token_type() {
            VEvent => EventBuilder::new().into(),
            VTodo => TodoBuilder::new().into(),
            VJournal => JournalBuilder::new().into(),
            VFreeBusy => FreeBusyBuilder::new().into(),
            _ => return Err(ParseError::UnknownComponent),
        };
        self.consume(Crlf, "expected crlf after BEGIN")?;
        // recursive call to property
        todo!()
    }

    fn property(&mut self) -> ParseResult<Vec<P>> {}

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
        let tt = self.next()?.token_type();
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

    #[error("unknown component")]
    UnknownComponent,

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

    #[error("The local time supplied did not yield a single time instance")]
    AmbiguousLocalTime,

    /// Shared by every value type backed by a `chrono` parser (`DATE`,
    /// `DATE-TIME`).
    #[error(transparent)]
    ChronoParse(#[from] chrono::ParseError),

    /// \[[Integer](crate::values::Integer)\] parsing error
    #[error("Malformed Integer")]
    Integer(#[from] std::num::ParseIntError),

    /// \[[Float](crate::values::Float)\] parsing error
    #[error("Malformed Float")]
    Float(#[from] std::num::ParseFloatError),

    /// \[[UtcOffset](crate::values::UtcOffset)\] parsing error
    #[error("Malformed UTC offset")]
    UtcOffset,

    /// \[[Duration](crate::values::Duration)\] parsing error
    #[error("Malformed Duration")]
    Duration,

    /// \[[Binary](crate::values::Binary)\] decoding error
    #[error("Malformed BASE64 data")]
    Base64(#[from] base64::DecodeError),
}
