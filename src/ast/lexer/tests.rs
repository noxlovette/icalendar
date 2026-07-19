use super::*;

/// `Token` has no accessors and derives only `Debug`, so equality here
/// is checked by comparing the derived Debug output against tokens
/// built with the same public [`Token::new`] constructor the lexer uses.
fn assert_tokens(actual: Vec<Token>, expected: Vec<Token>) {
    assert_eq!(format!("{:?}", actual), format!("{:?}", expected));
}

#[test]
fn scans_a_single_property_contentline() {
    let tokens = Lexer::new(b"BEGIN:VCALENDAR\r\n").scan().unwrap();

    // The Value token must hold only the value text after the `:`,
    // not the colon itself.
    assert_tokens(
        tokens,
        vec![
            Token::new(TokenType::Begin, b"BEGIN", None, 0),
            Token::new(TokenType::Colon, b":", None, 0),
            Token::new(TokenType::Value, b"VCALENDAR", Some(b"VCALENDAR"), 0),
            Token::new(TokenType::Crlf, b"\r\n", None, 0),
            Token::new(TokenType::Eof, b"", None, 1),
        ],
    );
}

#[test]
fn recognizes_rfc5545_keywords_case_insensitively() {
    let tokens = Lexer::new(b"RrUlE:FREQ=DAILY\r\n").scan().unwrap();
    assert_tokens(
        tokens,
        vec![
            Token::new(TokenType::RRule, b"RRULE", None, 0),
            Token::new(TokenType::Colon, b":", None, 0),
            Token::new(TokenType::Value, b"FREQ=DAILY", Some(b"FREQ=DAILY"), 0),
            Token::new(TokenType::Crlf, b"\r\n", None, 0),
            Token::new(TokenType::Eof, b"", None, 1),
        ],
    );
}

#[test]
fn unrecognized_name_becomes_identifier() {
    let tokens = Lexer::new(b"X-CUSTOM-PROP:value\r\n").scan().unwrap();
    assert_tokens(
        tokens,
        vec![
            Token::new(TokenType::Identifier, b"X-CUSTOM-PROP", None, 0),
            Token::new(TokenType::Colon, b":", None, 0),
            Token::new(TokenType::Value, b"value", Some(b"value"), 0),
            Token::new(TokenType::Crlf, b"\r\n", None, 0),
            Token::new(TokenType::Eof, b"", None, 1),
        ],
    );
}

#[test]
fn semicolon_param_with_equals_tokenizes_name_and_value() {
    let tokens =
        Lexer::new(b"RECURRENCE-ID;RANGE=THISANDFUTURE:20240402T100000\r\n")
            .scan()
            .unwrap();
    assert_tokens(
        tokens,
        vec![
            Token::new(TokenType::RecurrenceId, b"RECURRENCE-ID", None, 0),
            Token::new(TokenType::Semicolon, b";", None, 0),
            Token::new(TokenType::Range, b"RANGE", None, 0),
            Token::new(TokenType::Equals, b"=", None, 0),
            // Everything after `=` is param-value position, not name
            // position, so this is a ParamValue token even though
            // "THISANDFUTURE" also happens to not be a registered keyword.
            Token::new(
                TokenType::ParamValue,
                b"THISANDFUTURE",
                Some(b"THISANDFUTURE"),
                0,
            ),
            Token::new(TokenType::Colon, b":", None, 0),
            Token::new(
                TokenType::Value,
                b"20240402T100000",
                Some(b"20240402T100000"),
                0,
            ),
            Token::new(TokenType::Crlf, b"\r\n", None, 0),
            Token::new(TokenType::Eof, b"", None, 1),
        ],
    );
}

#[test]
fn line_advances_after_each_crlf() {
    // §3.1: a logical content line ends at CRLF. Every token scanned
    // after a CRLF belongs to the next (post-unfolding) line, so `line`
    // must increment once per CRLF consumed.
    let tokens = Lexer::new(b"UID:foo\r\nDTSTAMP:20240102T090000Z\r\n")
        .scan()
        .unwrap();
    assert_tokens(
        tokens,
        vec![
            Token::new(TokenType::Uid, b"UID", None, 0),
            Token::new(TokenType::Colon, b":", None, 0),
            Token::new(TokenType::Value, b"foo", Some(b"foo"), 0),
            Token::new(TokenType::Crlf, b"\r\n", None, 0),
            Token::new(TokenType::DtStamp, b"DTSTAMP", None, 1),
            Token::new(TokenType::Colon, b":", None, 1),
            Token::new(
                TokenType::Value,
                b"20240102T090000Z",
                Some(b"20240102T090000Z"),
                1,
            ),
            Token::new(TokenType::Crlf, b"\r\n", None, 1),
            Token::new(TokenType::Eof, b"", None, 2),
        ],
    );
}

#[test]
fn slash_in_raw_value_position_is_fine() {
    // After a `:`, everything up to CRLF is the property value verbatim.
    let tokens = Lexer::new(b"TZID:America/New_York\r\n").scan().unwrap();
    assert_tokens(
        tokens,
        vec![
            Token::new(TokenType::TzId, b"TZID", None, 0),
            Token::new(TokenType::Colon, b":", None, 0),
            Token::new(
                TokenType::Value,
                b"America/New_York",
                Some(b"America/New_York"),
                0,
            ),
            Token::new(TokenType::Crlf, b"\r\n", None, 0),
            Token::new(TokenType::Eof, b"", None, 1),
        ],
    );
}

#[test]
fn slash_in_unquoted_param_value_is_a_safe_char_not_an_error() {
    // §3.2: an unquoted param-value is `paramtext`, built from SAFE-CHAR,
    // which explicitly includes `/` (SAFE-CHAR excludes only CTL, DQUOTE,
    // `;`, `:`, `,`). A TZID param value like "America/New_York" is
    // ordinary, legal, unquoted param text and must not be rejected.
    let tokens =
        Lexer::new(b"DTSTART;TZID=America/New_York:20240104T100000\r\n")
            .scan()
            .unwrap();
    assert_tokens(
        tokens,
        vec![
            Token::new(TokenType::DtStart, b"DTSTART", None, 0),
            Token::new(TokenType::Semicolon, b";", None, 0),
            Token::new(TokenType::TzId, b"TZID", None, 0),
            Token::new(TokenType::Equals, b"=", None, 0),
            Token::new(
                TokenType::ParamValue,
                b"America/New_York",
                Some(b"America/New_York"),
                0,
            ),
            Token::new(TokenType::Colon, b":", None, 0),
            Token::new(
                TokenType::Value,
                b"20240104T100000",
                Some(b"20240104T100000"),
                0,
            ),
            Token::new(TokenType::Crlf, b"\r\n", None, 0),
            Token::new(TokenType::Eof, b"", None, 1),
        ],
    );
}

#[test]
fn comma_separated_param_values_all_tokenize_as_param_value() {
    // §3.2: param = param-name "=" param-value *("," param-value) — a
    // single param can carry a list of values. Every value in the list,
    // not just the first, is param-value position, so each one must come
    // out as ParamValue (never Identifier), and the `,` between them must
    // not reset param-value state the way `;` does.
    let tokens =
        Lexer::new(b"ATTENDEE;DELEGATED-FROM=A,B:mailto:foo@example.com\r\n")
            .scan()
            .unwrap();
    assert_tokens(
        tokens,
        vec![
            Token::new(TokenType::Attendee, b"ATTENDEE", None, 0),
            Token::new(TokenType::Semicolon, b";", None, 0),
            Token::new(TokenType::DelegatedFrom, b"DELEGATED-FROM", None, 0),
            Token::new(TokenType::Equals, b"=", None, 0),
            Token::new(TokenType::ParamValue, b"A", Some(b"A"), 0),
            Token::new(TokenType::Comma, b",", None, 0),
            Token::new(TokenType::ParamValue, b"B", Some(b"B"), 0),
            Token::new(TokenType::Colon, b":", None, 0),
            Token::new(
                TokenType::Value,
                b"mailto:foo@example.com",
                Some(b"mailto:foo@example.com"),
                0,
            ),
            Token::new(TokenType::Crlf, b"\r\n", None, 0),
            Token::new(TokenType::Eof, b"", None, 1),
        ],
    );
}

#[test]
fn quoted_string_param_value_is_supported() {
    // §3.2: param-value can be a quoted-string: DQUOTE *QSAFE-CHAR DQUOTE.
    // QSAFE-CHAR permits any character except CTL and DQUOTE, so a quoted
    // param-value may legally contain bytes (like the space here) that
    // would be illegal in unquoted paramtext. The resulting token's
    // lexeme/literal must hold the unquoted content, not the surrounding
    // DQUOTEs.
    let tokens =
        Lexer::new(b"ORGANIZER;CN=\"John Doe\":mailto:jdoe@example.com\r\n")
            .scan()
            .unwrap();
    assert_tokens(
        tokens,
        vec![
            Token::new(TokenType::Organizer, b"ORGANIZER", None, 0),
            Token::new(TokenType::Semicolon, b";", None, 0),
            Token::new(TokenType::Cn, b"CN", None, 0),
            Token::new(TokenType::Equals, b"=", None, 0),
            Token::new(
                TokenType::ParamValue,
                b"John Doe",
                Some(b"John Doe"),
                0,
            ),
            Token::new(TokenType::Colon, b":", None, 0),
            Token::new(
                TokenType::Value,
                b"mailto:jdoe@example.com",
                Some(b"mailto:jdoe@example.com"),
                0,
            ),
            Token::new(TokenType::Crlf, b"\r\n", None, 0),
            Token::new(TokenType::Eof, b"", None, 1),
        ],
    );
}

#[test]
fn lone_cr_followed_by_non_lf_errors_gracefully() {
    let res = Lexer::new(b"BEGIN\rX").scan();
    assert!(matches!(res, Err(LexerError::Crlf { line: 0 })));
}

#[test]
fn trailing_bare_cr_errors_gracefully_instead_of_panicking() {
    // A `\r` as the very last byte of the source, with no trailing `\n`,
    // is malformed input, not a memory-safety incident: the lexer must
    // return `LexerError::Crlf`, never read past the end of the buffer.
    let res = Lexer::new(b"BEGIN:VCALENDAR\r").scan();
    assert!(matches!(res, Err(LexerError::Crlf { .. })));
}
