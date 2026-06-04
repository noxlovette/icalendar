use crate::{ParseError, ParseResult};
use bytes::Bytes;

/// Splits a Bytes vector by given pattern
pub(crate) fn split_once(b: &Bytes, pat: u8) -> ParseResult<(&[u8], &[u8])> {
    b.iter()
        .position(|b| *b == pat)
        .map(|pos| (&b[..pos], &b[pos + 1..]))
        .ok_or(ParseError::Parameter {
            expected: pat.to_string(),
            received: std::str::from_utf8(b).ok().map(|s| s.into()),
        })
}

/// [Case-insensitively](https://datatracker.ietf.org/doc/html/rfc5545#section-3.5) matches the name to a given pattern
pub(crate) fn match_name(b: &[u8], pat: &[u8]) -> ParseResult<()> {
    if b.to_ascii_uppercase() != pat {
        Err(ParseError::Parameter {
            expected: std::str::from_utf8(pat).unwrap_or("Unknown pattern").into(),
            received: std::str::from_utf8(b).ok().map(|s| s.into()),
        })
    } else {
        Ok(())
    }
}

/// Checks if a given value is in quotes and returns that value with the quotes stripped
pub(crate) fn strip_quoted_string(v: &[u8]) -> ParseResult<&[u8]> {
    let needle = &[b'"'];

    v.strip_prefix(needle)
        .and_then(|s| s.strip_suffix(needle))
        .ok_or(ParseError::QuotedString)
}
