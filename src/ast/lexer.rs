use super::token::{Token, TokenType};
use crate::ast::token::TokenType::{Identifier, ParamValue, Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Unknown lexeme at line {line}, got {got}")]
    UnknownLexeme { line: usize, got: u8 },
    #[error("Broken CRLF at line {line}")]
    Crlf { line: usize },
    #[error("Unterminated quoted-string param value starting at line {line}")]
    UnterminatedQuotedString { line: usize },
}

#[derive(Debug, Default)]
pub struct Lexer<'a> {
    source: &'a [u8],
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
    /// Set on `=`, cleared on `;`/`:`. `,` leaves it untouched, since
    /// `param-value *("," param-value)` keeps scanning more values for
    /// the same param. While set, `textual()` produces `ParamValue`
    /// tokens instead of doing a keyword lookup.
    in_param_value: bool,
}

impl<'a> Lexer<'a> {
    /// creates a new [Lexer] out of a source
    pub fn new(src: &'a [u8]) -> Self {
        Self {
            source: src,
            ..Default::default()
        }
    }

    /// scans the source for tokens
    pub fn scan(mut self) -> Result<Vec<Token<'a>>, LexerError> {
        use super::token::TokenType::*;
        while !self.is_at_end() {
            self.start = self.current;
            let c = self.next();

            match c {
                b':' => {
                    self.add_token(Colon, None);
                    self.in_param_value = false;
                    self.start += 1;
                    self.value();
                }
                b';' => {
                    self.add_token(Semicolon, None);
                    self.in_param_value = false;
                }
                b',' => self.add_token(Comma, None),
                b'=' => {
                    self.add_token(Equals, None);
                    self.in_param_value = true;
                }
                b'"' => {
                    self.param_value()?;
                }
                b'\r' => {
                    if self.match_next(b'\n') {
                        self.add_token(Crlf, None);
                        self.line += 1;
                    } else {
                        return Err(LexerError::Crlf { line: self.line });
                    }
                }
                c if c.is_ascii_alphanumeric() => {
                    self.textual();
                }
                b'\n' | b' ' | b'\t' => {}
                _ => {
                    return Err(LexerError::UnknownLexeme {
                        line: self.line,
                        got: c,
                    });
                }
            }
        }

        self.tokens
            .push(Token::new(TokenType::Eof, b"", None, self.line));

        Ok(self.tokens)
    }

    fn value(&mut self) {
        let rest = &self.source[self.current..];
        self.current += memchr::memchr(b'\r', rest).unwrap_or(rest.len());

        let text = &self.source[self.start..self.current];

        self.add_token(Value, Some(text));
    }

    fn textual(&mut self) {
        while self.peek().is_ascii_alphanumeric()
            || matches!(self.peek(), b'-' | b'/' | b'_')
        {
            self.next();
        }

        let text = &self.source[self.start..self.current];

        if self.in_param_value {
            self.tokens.push(Token::new(
                ParamValue,
                text,
                Some(text),
                self.line,
            ));
            return;
        }

        // Real-world input follows the RFC 5545 §2 convention of writing
        // names in uppercase already, so only pay for the fold (and its
        // allocation) when the text actually contains a lowercase byte.
        if text.iter().any(u8::is_ascii_lowercase) {
            let upper = text.to_ascii_uppercase();
            match KEYWORDS.get(upper.as_slice()) {
                None => self
                    .tokens
                    .push(Token::new(Identifier, &upper, None, self.line)),
                Some(&tt) => {
                    self.tokens.push(Token::new(tt, &upper, None, self.line))
                }
            }
        } else {
            match KEYWORDS.get(text) {
                None => self
                    .tokens
                    .push(Token::new(Identifier, text, None, self.line)),
                Some(&tt) => {
                    self.tokens.push(Token::new(tt, text, None, self.line))
                }
            }
        }
    }

    fn param_value(&mut self) -> Result<(), LexerError> {
        // Skip the opening DQUOTE — it's not part of the value.
        self.start = self.current;

        let rest = &self.source[self.current..];
        match memchr::memchr(b'"', rest) {
            Some(offset) => self.current += offset,
            None => {
                self.current = self.source.len();
                return Err(LexerError::UnterminatedQuotedString {
                    line: self.line,
                });
            }
        }

        let text = &self.source[self.start..self.current];
        self.next(); // consume the closing DQUOTE

        self.tokens
            .push(Token::new(ParamValue, text, Some(text), self.line));
        Ok(())
    }

    /// adds a new token to self
    fn add_token(&mut self, tt: TokenType, lit: Option<&'a [u8]>) {
        let lex = &self.source[self.start..self.current];
        self.tokens.push(Token::new(tt, lex, lit, self.line));
    }

    /// advances the lexer, returning the consumed byte
    fn next(&mut self) -> u8 {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    /// looks at what the next byte is
    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source[self.current]
        }
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            false
        } else if self.source[self.current] == expected {
            self.current += 1;
            true
        } else {
            false
        }
    }

    /// checks that the lexer has read all of the source
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

static KEYWORDS: phf::Map<&'static [u8], TokenType> = phf::phf_map! {
    // --- component names, §3.6 ---
    b"BEGIN" => TokenType::Begin,
    b"END" => TokenType::End,
    b"VCALENDAR" => TokenType::VCalendar,
    b"VEVENT" => TokenType::VEvent,
    b"VTODO" => TokenType::VTodo,
    b"VJOURNAL" => TokenType::VJournal,
    b"VFREEBUSY" => TokenType::VFreeBusy,
    b"VTIMEZONE" => TokenType::VTimezone,
    b"VALARM" => TokenType::VAlarm,
    b"STANDARD" => TokenType::Standard,
    b"DAYLIGHT" => TokenType::Daylight,
    // --- calendar properties, §3.7 ---
    b"CALSCALE" => TokenType::CalScale,
    b"METHOD" => TokenType::Method,
    b"PRODID" => TokenType::ProdId,
    b"VERSION" => TokenType::Version,
    // --- descriptive properties, §3.8.1 ---
    b"ATTACH" => TokenType::Attach,
    b"CATEGORIES" => TokenType::Categories,
    b"CLASS" => TokenType::Class,
    b"COMMENT" => TokenType::Comment,
    b"DESCRIPTION" => TokenType::Description,
    b"GEO" => TokenType::Geo,
    b"LOCATION" => TokenType::Location,
    b"PERCENT-COMPLETE" => TokenType::PercentComplete,
    b"PRIORITY" => TokenType::Priority,
    b"RESOURCES" => TokenType::Resources,
    b"STATUS" => TokenType::Status,
    b"SUMMARY" => TokenType::Summary,
    // --- date and time properties, §3.8.2 ---
    b"COMPLETED" => TokenType::Completed,
    b"DTEND" => TokenType::DtEnd,
    b"DUE" => TokenType::Due,
    b"DTSTART" => TokenType::DtStart,
    b"DURATION" => TokenType::Duration,
    b"FREEBUSY" => TokenType::FreeBusy,
    b"TRANSP" => TokenType::Transp,
    // --- time zone properties, §3.8.3 ---
    b"TZID" => TokenType::TzId,
    b"TZNAME" => TokenType::TzName,
    b"TZOFFSETFROM" => TokenType::TzOffsetFrom,
    b"TZOFFSETTO" => TokenType::TzOffsetTo,
    b"TZURL" => TokenType::TzUrl,
    // --- relationship properties, §3.8.4 ---
    b"ATTENDEE" => TokenType::Attendee,
    b"CONTACT" => TokenType::Contact,
    b"ORGANIZER" => TokenType::Organizer,
    b"RECURRENCE-ID" => TokenType::RecurrenceId,
    b"RELATED-TO" => TokenType::RelatedTo,
    b"URL" => TokenType::Url,
    b"UID" => TokenType::Uid,
    // --- recurrence properties, §3.8.5 ---
    b"EXDATE" => TokenType::ExDate,
    b"RDATE" => TokenType::RDate,
    b"RRULE" => TokenType::RRule,
    // --- alarm properties, §3.8.6 ---
    b"ACTION" => TokenType::Action,
    b"REPEAT" => TokenType::Repeat,
    b"TRIGGER" => TokenType::Trigger,
    // --- change management properties, §3.8.7 ---
    b"CREATED" => TokenType::Created,
    b"DTSTAMP" => TokenType::DtStamp,
    b"LAST-MODIFIED" => TokenType::LastModified,
    b"SEQUENCE" => TokenType::Sequence,
    // --- miscellaneous properties, §3.8.8 ---
    b"REQUEST-STATUS" => TokenType::RequestStatus,
    // --- property parameters, §3.2 ---
    b"ALTREP" => TokenType::Altrep,
    b"CN" => TokenType::Cn,
    b"CUTYPE" => TokenType::CuType,
    b"DELEGATED-FROM" => TokenType::DelegatedFrom,
    b"DELEGATED-TO" => TokenType::DelegatedTo,
    b"DIR" => TokenType::Dir,
    b"ENCODING" => TokenType::Encoding,
    b"FMTTYPE" => TokenType::FmtType,
    b"FBTYPE" => TokenType::FbType,
    b"LANGUAGE" => TokenType::Language,
    b"MEMBER" => TokenType::Member,
    b"PARTSTAT" => TokenType::PartStat,
    b"RANGE" => TokenType::Range,
    b"RELATED" => TokenType::Related,
    b"RELTYPE" => TokenType::RelType,
    b"ROLE" => TokenType::Role,
    b"RSVP" => TokenType::Rsvp,
    b"SENT-BY" => TokenType::SentBy,
};

pub(crate) mod unfold;
pub use unfold::unfold;

#[cfg(test)]
mod tests;
