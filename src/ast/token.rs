/// Every lexical token produced by scanning an iCalendar content stream.
///
/// The grammar's only real *structure* the lexer needs to track is block
/// nesting ([Section 3.6](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6)):
/// `BEGIN`/`END` pairs, whose component name tells the parser which
/// builder to recurse into. Everything else on a content line — property
/// parameters, the value, DQUOTE-ing — is flat text that a property's own
/// `TryFrom<&[u8]>` (see `crate::properties`) parses directly from raw
/// bytes, using its own quote-aware splitting. So the lexer doesn't
/// tokenize params or values at all: every content line other than
/// `BEGIN`/`END` becomes a single [`Property`](TokenType::Property) token
/// carrying the property name and the raw, unparsed remainder of the
/// line, for a name → parser dispatch table to consume.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// `BEGIN`. The token's literal payload is the component name that
    /// follows the `:` (e.g. `VEVENT`), upper-cased.
    Begin,
    /// `END`. The token's literal payload is the component name that
    /// follows the `:`, upper-cased.
    End,
    /// Any content line other than `BEGIN`/`END`: a calendar property
    /// (§3.7), a component property (§3.8), or a non-standard/IANA
    /// extension property. The lexeme is the property name, upper-cased;
    /// the literal is everything from (but not including) that name to
    /// end of line — `*(";" param) ":" value` — completely unparsed, for
    /// the matching property type's `TryFrom<&[u8]>` to consume.
    Property,
    /// End of a (post-unfolding, logical) content line.
    Crlf,
    /// End of input.
    Eof,
}

/// A single scanned token: its classified [`TokenType`], the raw source
/// bytes it was scanned from, an optional decoded literal payload, and the
/// logical (post-unfolding) content-line number it starts on.
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    /// Bytes as scanned, normalized per [Section 2](https://datatracker.ietf.org/doc/html/rfc5545#section-2):
    /// names are case-insensitive, so keyword/name lexemes are upper-cased
    /// here rather than kept as a borrowed source subslice. Owned so a
    /// lowercase-in-the-wild lexeme (e.g. `b"dtstart"`) can be normalized
    /// to `b"DTSTART"` without aliasing the original source.
    lexeme: Vec<u8>,
    /// Decoded payload: for `Begin`/`End` this is the (upper-cased)
    /// component name; for `Property` this is the raw, unparsed remainder
    /// of the content line. `None` for `Crlf`/`Eof`.
    literal: Vec<u8>,
    line: usize,
}

impl Token {
    pub fn new(
        t: TokenType,
        lex: &[u8],
        lit: Option<&[u8]>,
        line: usize,
    ) -> Self {
        Self {
            token_type: t,
            lexeme: lex.to_vec(),
            literal: lit.map(|v| v.to_vec()).unwrap_or_default(),
            line,
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn literal(&self) -> &[u8] {
        self.literal.as_ref()
    }

    pub fn lexeme(&self) -> &[u8] {
        self.lexeme.as_ref()
    }

    pub fn lexeme_as_string(&self) -> String {
        String::from_utf8(self.lexeme.clone()).unwrap_or_default()
    }
}
