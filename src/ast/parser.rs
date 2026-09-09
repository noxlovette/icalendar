use super::token::Token;
use crate::{
    Calendar,
    ast::{
        CalendarBuilder, CalendarError, Component, EventBuilder,
        FreeBusyBuilder, JournalBuilder, Property, TodoBuilder,
        parse_property, token::TokenType,
    },
};
use TokenType::*;
use std::str::Utf8Error;
use thiserror::Error;

/// parses Tokens into valid iCal Formal Grammar
///
/// The grammar is recursive-descent, top-down, single-token lookahead:
/// [`Self::calendar`] recurses into [`Self::component`] on `BEGIN`, which
/// recurses further for nested components (`VALARM` inside `VEVENT`, etc.
/// — not yet wired up). There's no sub-line grammar left to descend into
/// here — a property line arrives from the lexer as one opaque
/// [`TokenType::Property`] token, and [`parse_property`] (backed by a
/// name -> parser dispatch table, see `crate::ast::PROPERTY_DISPATCH`)
/// hands back a fully-parsed [`Property`] in one step.
#[derive(Default, Debug)]
pub struct Parser {
    tokens: Vec<Token>,
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

    /// main entry point
    pub fn calendar(&mut self) -> ParseResult<Calendar> {
        let mut cal = CalendarBuilder::new();

        // Calendar properties (§3.7) precede any component.
        while !self.check(Begin)? {
            let prop =
                self.consume(Property, "expected a calendar property")?;
            let property = parse_property(prop.lexeme(), prop.literal())?;
            self.consume(Crlf, "expected crlf after property")?;

            match property {
                Property::ProductIdentifier(p) => cal.prodid = Some(p),
                Property::Version(v) => cal.version = Some(v),
                Property::Method(m) => cal.method = Some(m),
                Property::CalendarScale(c) => cal.calscale = Some(c),
                Property::Xprop(x) => cal.xprop.push(x),
                Property::Iana(i) => cal.iana.push(i),
                _ => return Err(ParseError::UnexpectedProperty),
            }
        }

        while self.check(Begin)? {
            cal.components.push(self.component()?);
        }

        todo!("consume the outer END:VCALENDAR and finish cal.build()")
    }

    /// parses one `BEGIN:<name> ... END:<name>` component, routing its
    /// property lines into the matching builder
    fn component(&mut self) -> ParseResult<Component> {
        let begin =
            self.consume(Begin, "expected component to start with BEGIN")?;
        let name = begin.literal().to_vec();

        let component: Component = match name.as_slice() {
            b"VEVENT" => EventBuilder::new().into(),
            b"VTODO" => TodoBuilder::new().into(),
            b"VJOURNAL" => JournalBuilder::new().into(),
            b"VFREEBUSY" => FreeBusyBuilder::new().into(),
            _ => return Err(ParseError::UnknownComponent),
        };
        self.consume(Crlf, "expected crlf after BEGIN")?;

        while !self.check(End)? {
            let prop = self.consume(Property, "expected a property line")?;
            let _property = parse_property(prop.lexeme(), prop.literal())?;
            self.consume(Crlf, "expected crlf after property")?;

            todo!("route `_property` into `component`'s builder fields")
        }

        let end =
            self.consume(End, "expected component to end with END")?;
        if end.literal() != name.as_slice() {
            return Err(ParseError::MismatchedEnd);
        }
        self.consume(Crlf, "expected crlf after END")?;

        Ok(component)
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

    #[error("property is not valid at this position in the grammar")]
    UnexpectedProperty,

    #[error("component's END name doesn't match its BEGIN name")]
    MismatchedEnd,

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
