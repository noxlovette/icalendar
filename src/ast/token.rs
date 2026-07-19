/// Every lexical token produced by scanning an iCalendar content stream.
///
/// Structural/generic tokens cover the contentline grammar itself
/// ([Section 3.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.1)).
/// All other variants are RFC 5545 keyword tokens: calendar/component
/// names (§3.6), calendar properties (§3.7), component properties (§3.8),
/// and property parameter names (§3.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // --- structural ---
    /// `:`
    Colon,
    /// `;`
    Semicolon,
    /// `,`
    Comma,
    /// `=`
    Equals,
    /// End of a (post-unfolding, logical) content line.
    Crlf,
    /// End of input.
    Eof,

    // --- generic terminals ---
    /// A non-standard (`X-`) or IANA-registered name in property/param
    /// position. §3.8.8.1 / §3.8.8.2 / §3.2 (x-param/iana-param).
    Identifier,
    /// The raw value text following `:` (a property value) or `=`
    /// (a parameter value).
    Value,

    // --- component names, §3.6 ---
    /// BEGIN. §3.6
    Begin,
    /// END. §3.6
    End,
    /// VCALENDAR. §3.4
    VCalendar,
    /// VEVENT. §3.6.1
    VEvent,
    /// VTODO. §3.6.2
    VTodo,
    /// VJOURNAL. §3.6.3
    VJournal,
    /// VFREEBUSY. §3.6.4
    VFreeBusy,
    /// VTIMEZONE. §3.6.5
    VTimezone,
    /// VALARM. §3.6.6
    VAlarm,
    /// STANDARD. §3.6.5
    Standard,
    /// DAYLIGHT. §3.6.5
    Daylight,

    // --- calendar properties, §3.7 ---
    /// CALSCALE. §3.7.1
    CalScale,
    /// METHOD. §3.7.2
    Method,
    /// PRODID. §3.7.3
    ProdId,
    /// VERSION. §3.7.4
    Version,

    // --- descriptive properties, §3.8.1 ---
    /// ATTACH. §3.8.1.1
    Attach,
    /// CATEGORIES. §3.8.1.2
    Categories,
    /// CLASS. §3.8.1.3
    Class,
    /// COMMENT. §3.8.1.4
    Comment,
    /// DESCRIPTION. §3.8.1.5
    Description,
    /// GEO. §3.8.1.6
    Geo,
    /// LOCATION. §3.8.1.7
    Location,
    /// PERCENT-COMPLETE. §3.8.1.8
    PercentComplete,
    /// PRIORITY. §3.8.1.9
    Priority,
    /// RESOURCES. §3.8.1.10
    Resources,
    /// STATUS. §3.8.1.11
    Status,
    /// SUMMARY. §3.8.1.12
    Summary,

    // --- date and time properties, §3.8.2 ---
    /// COMPLETED. §3.8.2.1
    Completed,
    /// DTEND. §3.8.2.2
    DtEnd,
    /// DUE. §3.8.2.3
    Due,
    /// DTSTART. §3.8.2.4
    DtStart,
    /// DURATION. §3.8.2.5
    Duration,
    /// FREEBUSY. §3.8.2.6
    FreeBusy,
    /// TRANSP. §3.8.2.7
    Transp,

    // --- time zone properties, §3.8.3 ---
    // TzId is shared with the §3.2.19 TZID *parameter* — same keyword text,
    // one variant; the parser disambiguates by grammar position.
    /// TZID. §3.8.3.1 (property) / §3.2.19 (parameter)
    TzId,
    /// TZNAME. §3.8.3.2
    TzName,
    /// TZOFFSETFROM. §3.8.3.3
    TzOffsetFrom,
    /// TZOFFSETTO. §3.8.3.4
    TzOffsetTo,
    /// TZURL. §3.8.3.5
    TzUrl,

    // --- relationship properties, §3.8.4 ---
    /// ATTENDEE. §3.8.4.1
    Attendee,
    /// CONTACT. §3.8.4.2
    Contact,
    /// ORGANIZER. §3.8.4.3
    Organizer,
    /// RECURRENCE-ID. §3.8.4.4
    RecurrenceId,
    /// RELATED-TO. §3.8.4.5
    RelatedTo,
    /// URL. §3.8.4.6
    Url,
    /// UID. §3.8.4.7
    Uid,

    // --- recurrence properties, §3.8.5 ---
    /// EXDATE. §3.8.5.1
    ExDate,
    /// RDATE. §3.8.5.2
    RDate,
    /// RRULE. §3.8.5.3
    RRule,

    // --- alarm properties, §3.8.6 ---
    /// ACTION. §3.8.6.1
    Action,
    /// REPEAT. §3.8.6.2
    Repeat,
    /// TRIGGER. §3.8.6.3
    Trigger,

    // --- change management properties, §3.8.7 ---
    /// CREATED. §3.8.7.1
    Created,
    /// DTSTAMP. §3.8.7.2
    DtStamp,
    /// LAST-MODIFIED. §3.8.7.3
    LastModified,
    /// SEQUENCE. §3.8.7.4
    Sequence,

    // --- miscellaneous properties, §3.8.8 ---
    /// REQUEST-STATUS. §3.8.8.3
    RequestStatus,

    // --- property parameters, §3.2 (excluding TzId, Value above) ---
    /// ALTREP. §3.2.1
    Altrep,
    /// CN. §3.2.2
    Cn,
    /// CUTYPE. §3.2.3
    CuType,
    /// DELEGATED-FROM. §3.2.4
    DelegatedFrom,
    /// DELEGATED-TO. §3.2.5
    DelegatedTo,
    /// DIR. §3.2.6
    Dir,
    /// ENCODING. §3.2.7
    Encoding,
    /// FMTTYPE. §3.2.8
    FmtType,
    /// FBTYPE. §3.2.9
    FbType,
    /// LANGUAGE. §3.2.10
    Language,
    /// MEMBER. §3.2.11
    Member,
    /// PARTSTAT. §3.2.12
    PartStat,
    /// RANGE. §3.2.13
    Range,
    /// RELATED. §3.2.14
    Related,
    /// RELTYPE. §3.2.15
    RelType,
    /// ROLE. §3.2.16
    Role,
    /// RSVP. §3.2.17
    Rsvp,
    /// SENT-BY. §3.2.18
    SentBy,
}

/// A single scanned token: its classified [`TokenType`], the raw source
/// bytes it was scanned from, an optional decoded literal payload, and the
/// logical (post-unfolding) content-line number it starts on.
#[derive(Debug)]
pub struct Token<'a> {
    token_type: TokenType,
    /// Raw bytes exactly as scanned (e.g. `b"DTSTART"`, `b"America/New_York"`).
    lexeme: &'a [u8],
    /// Decoded value payload — `Some` only for `Value`/`Identifier` tokens
    /// carrying a property or parameter value; `None` for keyword and
    /// structural tokens.
    literal: Option<&'a [u8]>,
    line: usize,
}

impl<'a> Token<'a> {
    pub fn new(
        t: TokenType,
        lex: &'a [u8],
        lit: Option<&'a [u8]>,
        line: usize,
    ) -> Self {
        Self {
            token_type: t,
            lexeme: lex,
            literal: lit,
            line,
        }
    }
}
