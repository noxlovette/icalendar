use super::token::{Token, TokenType};
use crate::ast::token::TokenType::{Identifier, Value};
use std::{collections::HashMap, sync::LazyLock};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Unknown lexeme at line {line}, got {got}")]
    UnknownLexeme { line: usize, got: String },
    #[error("Broken CRLF at line {line}")]
    Crlf { line: usize },
}

#[derive(Debug, Default)]
pub struct Lexer<'a> {
    source: &'a [u8],
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
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
                    self.value();
                }
                b';' => self.add_token(Semicolon, None),
                b',' => self.add_token(Comma, None),
                b'=' => self.add_token(Equals, None),
                b'\r' => {
                    if self.peek() == b'\n' {
                        self.add_token(Crlf, None);
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
                        got: "bullshit".into(),
                    });
                }
            }
        }

        self.tokens
            .push(Token::new(TokenType::Eof, b"", None, self.line));

        Ok(self.tokens)
    }

    fn value(&mut self) {
        while !(self.peek() == b'\r' && self.peek_next() == b'\n') {
            self.next();
        }

        let text = &self.source[self.start..self.current];

        self.add_token(Value, Some(text));
    }

    fn textual(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == b'-' {
            self.next();
        }

        let text = &self.source[self.start..self.current];

        let tt = match KEYWORDS.get(text) {
            None => Identifier,
            Some(&k) => k,
        };

        self.add_token(tt, None);
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

    fn peek_next(&self) -> u8 {
        if self.current >= self.source.len() {
            b'\0'
        } else {
            self.source[self.current + 1]
        }
    }

    /// checks that the lexer has read all of the source
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

static KEYWORDS: LazyLock<HashMap<&'static [u8], TokenType>> =
    LazyLock::new(|| {
        use super::token::TokenType::*;
        HashMap::from([
            // --- component names, §3.6 ---
            (b"BEGIN".as_slice(), Begin),
            (b"END".as_slice(), End),
            (b"VCALENDAR".as_slice(), VCalendar),
            (b"VEVENT".as_slice(), VEvent),
            (b"VTODO".as_slice(), VTodo),
            (b"VJOURNAL".as_slice(), VJournal),
            (b"VFREEBUSY".as_slice(), VFreeBusy),
            (b"VTIMEZONE".as_slice(), VTimezone),
            (b"VALARM".as_slice(), VAlarm),
            (b"STANDARD".as_slice(), Standard),
            (b"DAYLIGHT".as_slice(), Daylight),
            // --- calendar properties, §3.7 ---
            (b"CALSCALE".as_slice(), CalScale),
            (b"METHOD".as_slice(), Method),
            (b"PRODID".as_slice(), ProdId),
            (b"VERSION".as_slice(), Version),
            // --- descriptive properties, §3.8.1 ---
            (b"ATTACH".as_slice(), Attach),
            (b"CATEGORIES".as_slice(), Categories),
            (b"CLASS".as_slice(), Class),
            (b"COMMENT".as_slice(), Comment),
            (b"DESCRIPTION".as_slice(), Description),
            (b"GEO".as_slice(), Geo),
            (b"LOCATION".as_slice(), Location),
            (b"PERCENT-COMPLETE".as_slice(), PercentComplete),
            (b"PRIORITY".as_slice(), Priority),
            (b"RESOURCES".as_slice(), Resources),
            (b"STATUS".as_slice(), Status),
            (b"SUMMARY".as_slice(), Summary),
            // --- date and time properties, §3.8.2 ---
            (b"COMPLETED".as_slice(), Completed),
            (b"DTEND".as_slice(), DtEnd),
            (b"DUE".as_slice(), Due),
            (b"DTSTART".as_slice(), DtStart),
            (b"DURATION".as_slice(), Duration),
            (b"FREEBUSY".as_slice(), FreeBusy),
            (b"TRANSP".as_slice(), Transp),
            // --- time zone properties, §3.8.3 ---
            (b"TZID".as_slice(), TzId),
            (b"TZNAME".as_slice(), TzName),
            (b"TZOFFSETFROM".as_slice(), TzOffsetFrom),
            (b"TZOFFSETTO".as_slice(), TzOffsetTo),
            (b"TZURL".as_slice(), TzUrl),
            // --- relationship properties, §3.8.4 ---
            (b"ATTENDEE".as_slice(), Attendee),
            (b"CONTACT".as_slice(), Contact),
            (b"ORGANIZER".as_slice(), Organizer),
            (b"RECURRENCE-ID".as_slice(), RecurrenceId),
            (b"RELATED-TO".as_slice(), RelatedTo),
            (b"URL".as_slice(), Url),
            (b"UID".as_slice(), Uid),
            // --- recurrence properties, §3.8.5 ---
            (b"EXDATE".as_slice(), ExDate),
            (b"RDATE".as_slice(), RDate),
            (b"RRULE".as_slice(), RRule),
            // --- alarm properties, §3.8.6 ---
            (b"ACTION".as_slice(), Action),
            (b"REPEAT".as_slice(), Repeat),
            (b"TRIGGER".as_slice(), Trigger),
            // --- change management properties, §3.8.7 ---
            (b"CREATED".as_slice(), Created),
            (b"DTSTAMP".as_slice(), DtStamp),
            (b"LAST-MODIFIED".as_slice(), LastModified),
            (b"SEQUENCE".as_slice(), Sequence),
            // --- miscellaneous properties, §3.8.8 ---
            (b"REQUEST-STATUS".as_slice(), RequestStatus),
            // --- property parameters, §3.2 ---
            (b"ALTREP".as_slice(), Altrep),
            (b"CN".as_slice(), Cn),
            (b"CUTYPE".as_slice(), CuType),
            (b"DELEGATED-FROM".as_slice(), DelegatedFrom),
            (b"DELEGATED-TO".as_slice(), DelegatedTo),
            (b"DIR".as_slice(), Dir),
            (b"ENCODING".as_slice(), Encoding),
            (b"FMTTYPE".as_slice(), FmtType),
            (b"FBTYPE".as_slice(), FbType),
            (b"LANGUAGE".as_slice(), Language),
            (b"MEMBER".as_slice(), Member),
            (b"PARTSTAT".as_slice(), PartStat),
            (b"RANGE".as_slice(), Range),
            (b"RELATED".as_slice(), Related),
            (b"RELTYPE".as_slice(), RelType),
            (b"ROLE".as_slice(), Role),
            (b"RSVP".as_slice(), Rsvp),
            (b"SENT-BY".as_slice(), SentBy),
        ])
    });

#[cfg(test)]
mod tests;
