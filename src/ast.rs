use std::str::Utf8Error;
use thiserror::Error;
pub mod component;
pub mod params;
/// Splits a Bytes vector by given pattern
pub(crate) fn split_once(b: &[u8], needle: u8) -> ParseResult<(&[u8], &[u8])> {
    b.iter()
        .position(|b| *b == needle)
        .map(|pos| (&b[..pos], &b[pos + 1..]))
        .ok_or(ParseError::Parameter {
            expected: needle.to_string(),
            received: std::str::from_utf8(b).ok().map(|s| s.into()),
        })
}

/// [Case-insensitively](https://datatracker.ietf.org/doc/html/rfc5545#section-3.5) matches the name to a given pattern
pub(crate) fn match_name(b: &[u8], pat: &[u8]) -> ParseResult<()> {
    if b.to_ascii_uppercase() != pat {
        Err(ParseError::Parameter {
            expected: std::str::from_utf8(pat)
                .unwrap_or("Unknown pattern")
                .into(),
            received: std::str::from_utf8(b).ok().map(|s| s.into()),
        })
    } else {
        Ok(())
    }
}

/// Checks if a given value is in quotes and returns that value with the quotes
/// stripped
pub(crate) fn strip_quoted_string(v: &[u8]) -> ParseResult<&[u8]> {
    let needle = &[b'"'];

    v.strip_prefix(needle)
        .and_then(|s| s.strip_suffix(needle))
        .ok_or(ParseError::QuotedString)
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
}
