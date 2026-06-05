pub use crate::values::Recur;
use crate::{
    ParseError, ParseResult,
    internal::{match_name, split_once, strip_quoted_string},
    values::{Boolean, CalendarUserAddress, MediaType, Text, Uri},
};
use bytes::Bytes;
use chrono_tz::Tz;
use std::io::Write;

/// This trait defines a parameter as a functional entity
pub trait Parameter: Sized {
    /// Writes the param as output in bytes
    fn write(&self, w: impl Write) -> ParseResult<()>;
    /// Parses anything that implements Read and returns the parsed parameter
    fn parse(b: Bytes) -> ParseResult<Self>;
}

/// Any property parameter defined in Section 3.2 of RFC 5545.
///
/// A property can have attributes with which it is associated.
/// These "property parameters" contain meta-information about the
/// property or the property value.  Property parameters are specified
/// on a semicolon-separated list on the line of the property.
/// Property parameter values that contain the COLON, SEMICOLON, or
/// COMMA character separators MUST be specified as quoted-string
/// values.  Property values that do not contain any character
/// delimiters do not need to be quoted but may be.  Some property
/// parameter values are defined using a list of values.  In these
/// cases, the parameter value is defined as a COMMA-separated list of
/// values.
///
/// Example:
///
/// > DTSTART;TZID=America/New_York;VALUE=DATE-TIME:19980119T020000
///
/// [Section 3.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2)
pub enum PropertyParams {
    /// Alternate text representation URI (`ALTREP`). §3.2.1
    Altrep(Altrep),
    /// Common name for a calendar user (`CN`). §3.2.2
    CommonName(CommonName),
    /// Calendar user type (`CUTYPE`). §3.2.3
    CalendarUserType(CalendarUserType),
    /// Delegators — who delegated to this attendee (`DELEGATED-FROM`). §3.2.4
    Delegators(Delegators),
    /// Delegatees — to whom this attendee delegated (`DELEGATED-TO`). §3.2.5
    Delegatees(Delegatees),
    /// Directory entry reference URI (`DIR`). §3.2.6
    DirectoryEntryReference(DirectoryEntryReference),
    /// Inline encoding scheme (`ENCODING`). §3.2.7
    Encoding(Encoding),
    /// MIME media type of a referenced object (`FMTTYPE`). §3.2.8
    Fmttype(Fmttype),
    /// Free/busy time type (`FBTYPE`). §3.2.9
    Fbtype(Fbtype),
    /// Language of property text (`LANGUAGE`). §3.2.10
    Language(Language),
    /// Group/list membership addresses (`MEMBER`). §3.2.11
    Member(Member),
    /// Participation status (`PARTSTAT`). §3.2.12
    ParticipationStatus(ParticipationStatus),
    /// Recurrence identifier range (`RANGE`). §3.2.13
    RecurrenceIdentifierRange(RecurrenceIdentifierRange),
    /// Alarm trigger relationship (`RELATED`). §3.2.14
    AlarmTriggerRelationship(AlarmTriggerRelationship),
    /// Hierarchical relationship type (`RELTYPE`). §3.2.15
    RelationshipType(RelationshipType),
    /// Participation role (`ROLE`). §3.2.16
    ParticipationRole(ParticipationRole),
    /// Reply requested flag (`RSVP`). §3.2.17
    Rsvp(Rsvp),
    /// Acting-on-behalf-of address (`SENT-BY`). §3.2.18
    SentBy(SentBy),
    /// Time zone identifier (`TZID`). §3.2.19
    TimeZoneIdentifier(TimeZoneIdentifier),
    /// Explicit value type (`VALUE`). §3.2.20
    DataTypes(DataTypes),
    /// Non-standard `X-`-prefixed parameter.
    XName {
        name: crate::values::Text,
        value: crate::values::Text,
    },
    /// IANA-registered parameter not listed above.
    Iana {
        name: crate::values::Text,
        value: crate::values::Text,
    },
}

impl PropertyParams {
    fn parse(b: &Bytes) -> ParseResult<Vec<Self>> {
        // need to split by SEMICOLON BUT a SEMICOLON that is NOT in DOUBLE QUOTES
        todo!()
    }
}

pub use internal::*;
mod internal {
    use crate::{
        params::{Altrep, DataTypes, Language, Parameter, TimeZoneIdentifier},
        values::Text,
    };
    use std::collections::HashMap;

    /// Convenience wrapper around params
    #[derive(Debug, Default)]
    pub struct Params<T> {
        /// Types defined in the RFC
        standard: T,
        /// IANA-registered types
        iana: HashMap<Text, Text>,
        /// X-Name Types
        experimental: HashMap<Text, Text>,
    }

    /// Convenience type that multiple components share
    #[derive(Default)]
    pub struct TextParams {
        altrep: Option<Altrep>,
        language: Option<Language>,
    }

    /// Convenience parameter bundle shared by date/time properties (`DTSTART`,
    /// `DTEND`, etc.).
    #[derive(Default)]
    pub struct TimeParams {
        vtype: Option<DataTypes>,
        tzid: Option<TimeZoneIdentifier>,
    }
}

/// Explicit value type for a property, as carried by the `VALUE` parameter.
///
/// This parameter specifies the value type and format of the property value.
/// The property values MUST be of a single value type.  For example, on the
/// "DTSTART" property the value type defaults to DATE-TIME.  However, if
/// the value type is set to DATE, then the value MUST be a DATE value type.
///
/// Example:
///
/// > DTSTART;VALUE=DATE:19980101
///
/// [Section 3.2.20](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.20)
pub enum DataTypes {
    /// Inline binary data encoded as BASE64.
    Binary,
    /// A URI reference (RFC 3986).
    Uri,
    /// Plain text, with BACKSLASH escaping for special characters.
    Text,
    /// `TRUE` or `FALSE`.
    Boolean,
    /// A calendar user address (`mailto:` URI).
    CalAddress,
    /// An ISO 8601 calendar date.
    Date,
    /// An ISO 8601 calendar date and time of day.
    DateTime,
    /// An ISO 8601 duration.
    Duration,
    /// A floating-point number.
    Float,
    /// A signed 32-bit integer.
    Integer,
    /// A time period (start/end or start/duration).
    Period,
    /// A recurrence rule (`RRULE`).
    Recur,
    /// An ISO 8601 time of day.
    Time,
    /// A UTC offset (e.g. `-0500`).
    UtcOffset,
    /// A non-standard `X-` prefixed type name.
    XName(Text),
    /// An IANA-registered type name.
    Iana(Text),
}

impl Parameter for DataTypes {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"VALUE")?;

        let r = match v {
            b"BINARY" => Self::Binary,
            b"URI" => Self::Uri,
            b"TEXT" => Self::Text,
            b"BOOLEAN" => Self::Boolean,
            b"CAL-ADDRESS" => Self::CalAddress,
            b"DATE" => Self::Date,
            b"DATE-TIME" => Self::DateTime,
            b"DURATION" => Self::Duration,
            b"FLOAT" => Self::Float,
            b"INTEGER" => Self::Integer,
            b"PERIOD" => Self::Period,
            b"RECUR" => Self::Recur,
            b"TIME" => Self::Time,
            b"UTC-OFFSET" => Self::UtcOffset,
            x => {
                if x.to_ascii_uppercase().starts_with(b"X-") {
                    Self::XName(x.try_into()?)
                } else {
                    Self::Iana(x.try_into()?)
                }
            }
        };

        Ok(r)
    }

    fn write(&self, w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

/// This parameter specifies a URI that points to an
/// alternate representation for a textual property value.  A property
/// specifying this parameter MUST also include a value that reflects
/// the default representation of the text value.  The URI parameter
/// value MUST be specified in a quoted-string.
///
/// > Note: While there is no restriction imposed on the URI schemes
/// > allowed for this parameter, Content Identifier (CID) [RFC2392],
/// > HTTP [RFC2616], and HTTPS [RFC2818] are the URI schemes most
/// > commonly used by current implementations.
///
/// [Section 3.2.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.1)
pub struct Altrep(Uri);

impl Parameter for Altrep {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"ALTREP")?;
        let stripped = strip_quoted_string(v)?;

        Ok(Self(stripped.try_into()?))
    }

    fn write(&self, w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

/// This parameter can be specified on properties with a
/// CAL-ADDRESS value type.  The parameter specifies the common name
/// to be associated with the calendar user specified by the property.
/// The parameter value is text.  The parameter value can be used for
/// display text to be associated with the calendar address specified
/// by the property.
///
/// Example:
///
/// > ORGANIZER;CN="John Smith":mailto:jsmith@example.com
///
/// [Section 3.2.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.2)
pub struct CommonName(Text);

impl Parameter for CommonName {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"CN")?;

        Ok(Self(strip_quoted_string(v)?.try_into()?))
    }

    fn write(&self, w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

/// This parameter can be specified on properties with a
/// CAL-ADDRESS value type.  This parameter specifies those calendar
/// users that have delegated their participation in a group-scheduled
/// event or to-do to the calendar user specified by the property.
/// The individual calendar address parameter values MUST each be
/// specified in a quoted-string.
///
/// Example:
///
/// > ATTENDEE;DELEGATED-FROM="mailto:jsmith@example.com":mailto:jdoe@example.
/// > com
///
/// [Section 3.2.4](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.4)
pub struct Delegators(Vec<CalendarUserAddress>);

impl Parameter for Delegators {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"DELEGATED-FROM")?;

        let mut vec = Vec::new();
        for s in v.split(|b| *b == b',') {
            let stripped = strip_quoted_string(s)?;
            vec.push(stripped.try_into()?);
        }

        Ok(Self(vec))
    }

    fn write(&self, w: impl Write) -> ParseResult<()> {
        todo!()
    }
}
/// This parameter can be specified on properties with a
/// CAL-ADDRESS value type.  This parameter specifies those calendar
/// users whom have been delegated participation in a group-scheduled
/// event or to-do by the calendar user specified by the property.
/// The individual calendar address parameter values MUST each be
/// specified in a quoted-string.
///
/// Example:
///
/// > ATTENDEE;DELEGATED-TO="mailto:jdoe@example.com","mailto:jqpublic
/// > @example.com":mailto:jsmith@example.com
///
/// [Section 3.2.5](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.5)
pub struct Delegatees(Vec<CalendarUserAddress>);

impl Parameter for Delegatees {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"DELEGATED-TO")?;

        let mut vec = Vec::new();
        for s in v.split(|b| *b == b',') {
            let stripped = strip_quoted_string(s)?;
            vec.push(stripped.try_into()?);
        }

        Ok(Self(vec))
    }

    fn write(&self, w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

/// This parameter specifies a reference to a directory entry associated with
/// the calendar user specified by the property.  The parameter value is a
/// URI.  The URI parameter value MUST be specified in a quoted-string.
///
/// Example:
///
/// > ORGANIZER;DIR="ldap://example.com:6666/o=ABC%20Industries,c=US???(cn=Jim%
/// > 20Dolittle)":mailto:jimdo@example.com
///
/// [Section 3.2.6](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.6)
#[derive(Debug)]
pub struct DirectoryEntryReference(Uri);

impl Parameter for DirectoryEntryReference {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"DIR")?;
        let stripped = strip_quoted_string(v)?;

        Ok(Self(stripped.try_into()?))
    }

    fn write(&self, w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

/// This property parameter identifies the inline encoding
/// used in a property value.  The default encoding is "8BIT",
/// corresponding to a property value consisting of text.  The
/// "BASE64" encoding type corresponds to a property value encoded
/// using the "BASE64" encoding defined in [RFC2045](https://datatracker.ietf.org/doc/html/rfc2045).
/// If the value type parameter is ";VALUE=BINARY", then the inline
/// encoding parameter MUST be specified with the value
/// ";ENCODING=BASE64".
///
/// [Section 3.2.7](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.7)
#[derive(Debug, Default)]
pub enum Encoding {
    /// Default 8-bit text encoding.
    Bit8,
    /// BASE64 binary encoding, required when `VALUE=BINARY`.
    #[default]
    Base64,
}

impl Parameter for Encoding {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"ENCODING")?;

        let r = match v {
            b"BASE64" => Self::Base64,
            b"8BIT" => Self::Bit8,
            _ => {
                return Err(ParseError::Parameter {
                    expected: "BASE64 or 8BIT".into(),
                    received: std::str::from_utf8(b.as_ref())
                        .ok()
                        .map(|s| s.into()),
                });
            }
        };

        Ok(r)
    }

    fn write(&self, w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

/// This parameter can be specified on properties that are
/// used to reference an object.  The parameter specifies the media
/// type [RFC4288] of the referenced object.  For example, on the
/// "ATTACH" property, an FTP type URI value does not, by itself,
/// necessarily convey the type of content associated with the
/// resource.  The parameter value MUST be the text for either an
/// IANA-registered media type or a non-standard media type.
///
/// Example:
///
/// > ATTACH;FMTTYPE=application/msword:ftp://example.com/pub/docs/agenda.doc
///
/// TODO: replace with MIME
///
/// [Section 3.2.8](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.8)
pub struct Fmttype(MediaType);

impl Parameter for Fmttype {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"FMTTYPE")?;

        Ok(Self(v.try_into()?))
    }

    fn write(&self, w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

/// This parameter specifies the free or busy time type.
/// The value FREE indicates that the time interval is free for
/// scheduling.  The value BUSY indicates that the time interval is
/// busy because one or more events have been scheduled for that
/// interval.  The value BUSY-UNAVAILABLE indicates that the time
/// interval is busy and that the interval can not be scheduled.  The
/// value BUSY-TENTATIVE indicates that the time interval is busy
/// because one or more events have been tentatively scheduled for
/// that interval.  If not specified on a property that allows this
/// parameter, the default is BUSY.  Applications MUST treat x-name
/// and iana-token values they don't recognize the same way as they
/// would the BUSY value.
///
/// Example:
///
/// > FREEBUSY;FBTYPE=BUSY:19980415T133000Z/19980415T170000Z
///
/// [Section 3.2.9](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.9)
#[derive(Default, Debug)]
pub enum Fbtype {
    /// The interval is free for scheduling.
    Free,
    /// The interval is busy (one or more events scheduled).
    #[default]
    Busy,
    /// The interval is busy and cannot be scheduled.
    BusyUnavailable,
    /// The interval is tentatively busy.
    BusyTentative,
    X(Text),
    Iana(Text),
}

impl Parameter for Fbtype {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"FBTYPE")?;

        let r = match v {
            b"FREE" => Self::Free,
            b"BUSY" => Self::Busy,
            b"BUSY-UNAVAILABLE" => Self::BusyUnavailable,
            b"BUSY-TENTATIVE" => Self::BusyTentative,
            x => {
                if x.to_ascii_uppercase().starts_with(b"X-") {
                    Self::X(x.try_into()?)
                } else {
                    Self::Iana(x.try_into()?)
                }
            }
        };

        Ok(r)
    }

    fn write(&self, w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

/// This parameter identifies the language of the text in
/// the property value and of all property parameter values of the
/// property.  The value of the "LANGUAGE" property parameter is that
/// defined in [RFC5646].
///
/// For transport in a MIME entity, the Content-Language header field
/// can be used to set the default language for the entire body part.
/// Otherwise, no default language is assumed.
///
/// The following are examples of this parameter on the
/// "SUMMARY" and "LOCATION" properties:
///
/// > SUMMARY;LANGUAGE=en-US:Company Holiday Party
/// >
/// > LOCATION;LANGUAGE=en:Germany
/// >
/// > LOCATION;LANGUAGE=no:Tyskland
///
/// [Section 3.2.10](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.10)
pub struct Language(langtag::LangTagBuf);

impl Parameter for Language {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"LANGUAGE")?;
        Ok(Self(
            langtag::LangTagBuf::from_bytes(v.to_vec())
                .map_err(|_| ParseError::Language)?,
        ))
    }

    fn write(&self, w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

/// This parameter can be specified on properties with a
/// CAL-ADDRESS value type.  The parameter identifies the groups or
/// list membership for the calendar user specified by the property.
/// The parameter value is either a single calendar address in a
/// quoted-string or a COMMA-separated list of calendar addresses,
/// each in a quoted-string.  The individual calendar address
/// parameter values MUST each be specified in a quoted-string.
///
/// [Section 3.2.11](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.11)
pub struct Member(Vec<CalendarUserAddress>);

impl Parameter for Member {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"MEMBER")?;

        let mut vec = Vec::new();
        for el in v.split(|b| *b == b',') {
            vec.push(strip_quoted_string(el)?.try_into()?);
        }

        Ok(Self(vec))
    }

    fn write(&self, w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

/// This parameter can be specified on properties with a
/// CAL-ADDRESS value type.  The parameter identifies the type of
/// calendar user specified by the property.  If not specified on a
/// property that allows this parameter, the default is INDIVIDUAL.
/// Applications MUST treat x-name and iana-token values they don't
/// recognize the same way as they would the UNKNOWN value.
///
/// [Section 3.2.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.3)
#[derive(Debug, Default)]
pub enum CalendarUserType {
    /// A single person. Default.
    #[default]
    Individual,
    /// A group of calendar users.
    Group,
    /// A physical resource (e.g. a projector).
    Resource,
    /// A room resource.
    Room,
    /// The type is unknown.
    Unknown,
    X(Text),
    Iana(Text),
}

/// This parameter can be specified on properties with a
/// CAL-ADDRESS value type.  The parameter identifies the
/// participation status for the calendar user specified by the
/// property value.  The parameter values differ depending on whether
/// they are associated with a group-scheduled "VEVENT", "VTODO", or
/// "VJOURNAL".  The values MUST match one of the values allowed for
/// the given calendar component.  If not specified on a property that
/// allows this parameter, the default value is NEEDS-ACTION.
/// Applications MUST treat x-name and iana-token values they don't
/// recognize the same way as they would the NEEDS-ACTION value.
///
/// Example:
///
/// > ATTENDEE;PARTSTAT=DECLINED:mailto:jsmith@example.com
///
/// [Section 3.2.12](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.12)
#[derive(Debug)]
pub enum ParticipationStatus {
    /// Status for a `VEVENT` attendee.
    Event(PartStatEvent),
    /// Status for a `VTODO` attendee.
    Todo(PartStatTodo),
    /// Status for a `VJOURNAL` attendee.
    Journal(PartStatJournal),
    X(Text),
    Iana(Text),
}

/// This parameter can be specified on properties with a
/// CAL-ADDRESS value type.  The parameter identifies the expectation
/// of a reply from the calendar user specified by the property value.
/// This parameter is used by the "Organizer" to request a
/// participation status reply from an "Attendee" of a group-scheduled
/// event or to-do.  If not specified on a property that allows this
/// parameter, the default value is FALSE.
///
/// Example:
///
/// > ATTENDEE;RSVP=TRUE:mailto:jsmith@example.com
///
/// [Section 3.2.17](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.17)
pub struct Rsvp(Boolean);

/// This parameter can be specified on properties with a
/// CAL-ADDRESS value type.  The parameter specifies the calendar user
/// that is acting on behalf of the calendar user specified by the
/// property.  The parameter value MUST be a mailto URI as defined in
/// [RFC2368].  The individual calendar address parameter values MUST
/// each be specified in a quoted-string.
///
/// [Section 3.2.18](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.18)
pub struct SentBy(CalendarUserAddress);

/// This parameter MUST be specified on the "DTSTART",
/// "DTEND", "DUE", "EXDATE", and "RDATE" properties when either a
/// DATE-TIME or TIME value type is specified and when the value is
/// neither a UTC or a "floating" time.  Refer to the DATE-TIME or
/// TIME value type definition for a description of UTC and "floating
/// time" formats.  This property parameter specifies a text value
/// that uniquely identifies the "VTIMEZONE" calendar component to be
/// used when evaluating the time portion of the property.  The value
/// of the "TZID" property parameter will be equal to the value of the
/// "TZID" property for the matching time zone definition.  An
/// individual "VTIMEZONE" calendar component MUST be specified for
/// each unique "TZID" parameter value specified in the iCalendar
/// object.
///
/// The parameter MUST be specified on properties with a DATE-TIME
/// value if the DATE-TIME is not either a UTC or a "floating" time.
/// Failure to include and follow VTIMEZONE definitions in iCalendar
/// objects may lead to inconsistent understanding of the local time
/// at any given location.
///
/// The presence of the SOLIDUS character as a prefix, indicates that
/// this "TZID" represents a unique ID in a globally defined time zone
/// registry (when such registry is defined).
///
/// > Note: This document does not define a naming convention for
/// > time zone identifiers.  Implementers may want to use the naming
/// > conventions defined in existing time zone specifications such
/// > as the public-domain TZ database [TZDB].  The specification of
/// > globally unique time zone identifiers is not addressed by this
/// > document and is left for future study.
///
/// The following are examples of this property parameter:
///
/// > DTSTART;TZID=America/New_York:19980119T020000
/// >
/// > DTEND;TZID=America/New_York:19980119T030000
///
/// The "TZID" property parameter MUST NOT be applied to DATE
/// properties and DATE-TIME or TIME properties whose time values are
/// specified in UTC.
///
/// The use of local time in a DATE-TIME or TIME value without the
/// "TZID" property parameter is to be interpreted as floating time,
/// regardless of the existence of "VTIMEZONE" calendar components in
/// the iCalendar object.
///
/// For more information, see the sections on the value types [DateType] and
/// [Time].
///
/// [Section 3.2.19](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.19)
#[derive(Debug, Clone)]
pub struct TimeZoneIdentifier(Tz);

/// This parameter can be specified on properties with a
/// CAL-ADDRESS value type.  The parameter specifies the participation
/// role for the calendar user specified by the property in the group
/// schedule calendar component.  If not specified on a property that
/// allows this parameter, the default value is REQ-PARTICIPANT.
/// Applications MUST treat x-name and iana-token values they don't
/// recognize the same way as they would the REQ-PARTICIPANT value.
///
/// Example:
///
/// > ATTENDEE;ROLE=CHAIR:mailto:mrbig@example.com
///
/// [RFC](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.16)
#[derive(Debug, Default)]
pub enum ParticipationRole {
    /// Organizer/chair of the meeting.
    Chair,
    /// Required participant. Default.
    #[default]
    ReqParticipant,
    /// Optional participant.
    OptParticipant,
    /// Receives a copy but is not expected to participate.
    NonParticipant,
    X(Text),
    Iana(Text),
}

/// Participation statuses for a "VEVENT"
#[derive(Debug, Default)]
pub enum PartStatEvent {
    /// No reply has been received. Default.
    #[default]
    NeedsAction,
    /// Invitation has been accepted.
    Accepted,
    /// Invitation has been declined.
    Declined,
    /// Participation is tentative.
    Tentative,
    /// Participation has been delegated to another attendee.
    Delegated,
    X(Text),
    Iana(Text),
}

/// Participation statuses for a "VTODO"
#[derive(Debug, Default)]
pub enum PartStatTodo {
    /// No reply has been received. Default.
    #[default]
    NeedsAction,
    /// To-do has been accepted.
    Accepted,
    /// To-do has been declined.
    Declined,
    /// Participation is tentative.
    Tentative,
    /// To-do has been delegated.
    Delegated,
    /// To-do has been completed.
    Completed,
    /// To-do is being worked on.
    InProcess,
    X(Text),
    Iana(Text),
}

/// Participation statuses for a "VJOURNAL"
#[derive(Debug, Default)]
pub enum PartStatJournal {
    /// No reply has been received. Default.
    #[default]
    NeedsAction,
    /// Journal entry has been accepted.
    Accepted,
    /// Journal entry has been declined.
    Declined,
    X(Text),
    Iana(Text),
}

/// This parameter can be specified on a property that
/// references another related calendar.  The parameter specifies the
/// hierarchical relationship type of the calendar component
/// referenced by the property.  The parameter value can be PARENT, to
/// indicate that the referenced calendar component is a superior of
/// calendar component; CHILD to indicate that the referenced calendar
/// component is a subordinate of the calendar component; or SIBLING
/// to indicate that the referenced calendar component is a peer of
/// the calendar component.  If this parameter is not specified on an
/// allowable property, the default relationship type is PARENT.
/// Applications MUST treat x-name and iana-token values they don't
/// recognize the same way as they would the PARENT value.
///
/// Example:
///
/// > RELATED-TO;RELTYPE=SIBLING:19960401-080045-4000F192713@example.com
///
/// [Section 3.2.15](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.15)
#[derive(Debug, Default)]
pub enum RelationshipType {
    /// The referenced component is a parent (superior). Default.
    #[default]
    Parent,
    /// The referenced component is a child (subordinate).
    Child,
    /// The referenced component is a sibling (peer).
    Sibling,
    X(Text),
    Iana(Text),
}

/// This parameter can be specified on properties that
/// specify an alarm trigger with a "DURATION" value type.  The
/// parameter specifies whether the alarm will trigger relative to the
/// start or end of the calendar component.  The parameter value START
/// will set the alarm to trigger off the start of the calendar
/// component; the parameter value END will set the alarm to trigger
/// off the end of the calendar component.  If the parameter is not
/// specified on an allowable property, then the default is START.
///
/// Example:
///
/// > TRIGGER;RELATED=END:PT5M
///
/// [Section 3.2.14](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.14)
#[derive(Debug, Default)]
pub enum AlarmTriggerRelationship {
    /// Trigger relative to the start of the component. Default.
    #[default]
    Start,
    /// Trigger relative to the end of the component.
    End,
}

/// This parameter can be specified on a property that
/// specifies a recurrence identifier.  The parameter specifies the
/// effective range of recurrence instances that is specified by the
/// property.  The effective range is from the recurrence identifier
/// specified by the property.  If this parameter is not specified on
/// an allowed property, then the default range is the single instance
/// specified by the recurrence identifier value of the property.  The
/// parameter value can only be "THISANDFUTURE" to indicate a range
/// defined by the recurrence identifier and all subsequent instances.
/// The value "THISANDPRIOR" is deprecated by this revision of
/// iCalendar and MUST NOT be generated by applications.
///
/// Example:
///
/// > RECURRENCE-ID;RANGE=THISANDFUTURE:19980401T133000Z
///
/// [Section 3.2.13](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.13)
pub enum RecurrenceIdentifierRange {
    /// Range covers the identified instance and all subsequent instances.
    ThisAndFuture,
}

/// Shorthand alias for [`RecurrenceIdentifierRange`], used by
/// [`crate::RecurrenceSet`].
pub type Range = RecurrenceIdentifierRange;

impl Parameter for CalendarUserType {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"CUTYPE")?;
        let r = match v {
            b"INDIVIDUAL" => Self::Individual,
            b"GROUP" => Self::Group,
            b"RESOURCE" => Self::Resource,
            b"ROOM" => Self::Room,
            b"UNKNOWN" => Self::Unknown,
            x => {
                if x.to_ascii_uppercase().starts_with(b"X-") {
                    Self::X(x.try_into()?)
                } else {
                    Self::Iana(x.try_into()?)
                }
            }
        };
        Ok(r)
    }

    fn write(&self, _w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

impl Parameter for ParticipationRole {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"ROLE")?;
        let r = match v {
            b"CHAIR" => Self::Chair,
            b"REQ-PARTICIPANT" => Self::ReqParticipant,
            b"OPT-PARTICIPANT" => Self::OptParticipant,
            b"NON-PARTICIPANT" => Self::NonParticipant,
            x => {
                if x.to_ascii_uppercase().starts_with(b"X-") {
                    Self::X(x.try_into()?)
                } else {
                    Self::Iana(x.try_into()?)
                }
            }
        };
        Ok(r)
    }

    fn write(&self, _w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

impl Parameter for PartStatEvent {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"PARTSTAT")?;
        let r = match v {
            b"NEEDS-ACTION" => Self::NeedsAction,
            b"ACCEPTED" => Self::Accepted,
            b"DECLINED" => Self::Declined,
            b"TENTATIVE" => Self::Tentative,
            b"DELEGATED" => Self::Delegated,
            x => {
                if x.to_ascii_uppercase().starts_with(b"X-") {
                    Self::X(x.try_into()?)
                } else {
                    Self::Iana(x.try_into()?)
                }
            }
        };
        Ok(r)
    }

    fn write(&self, _w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

impl Parameter for PartStatTodo {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"PARTSTAT")?;
        let r = match v {
            b"NEEDS-ACTION" => Self::NeedsAction,
            b"ACCEPTED" => Self::Accepted,
            b"DECLINED" => Self::Declined,
            b"TENTATIVE" => Self::Tentative,
            b"DELEGATED" => Self::Delegated,
            b"COMPLETED" => Self::Completed,
            b"IN-PROCESS" => Self::InProcess,
            x => {
                if x.to_ascii_uppercase().starts_with(b"X-") {
                    Self::X(x.try_into()?)
                } else {
                    Self::Iana(x.try_into()?)
                }
            }
        };
        Ok(r)
    }

    fn write(&self, _w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

impl Parameter for PartStatJournal {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"PARTSTAT")?;
        let r = match v {
            b"NEEDS-ACTION" => Self::NeedsAction,
            b"ACCEPTED" => Self::Accepted,
            b"DECLINED" => Self::Declined,
            x => {
                if x.to_ascii_uppercase().starts_with(b"X-") {
                    Self::X(x.try_into()?)
                } else {
                    Self::Iana(x.try_into()?)
                }
            }
        };
        Ok(r)
    }

    fn write(&self, _w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

impl Parameter for Rsvp {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"RSVP")?;
        Ok(Self(v.try_into()?))
    }

    fn write(&self, _w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

impl Parameter for SentBy {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"SENT-BY")?;
        let stripped = strip_quoted_string(v)?;
        Ok(Self(stripped.try_into()?))
    }

    fn write(&self, _w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

impl Parameter for TimeZoneIdentifier {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"TZID")?;
        let s = std::str::from_utf8(v)?;
        let tz: Tz = s.parse().map_err(|_| ParseError::Parameter {
            expected: "IANA timezone identifier".into(),
            received: Some(s.into()),
        })?;
        Ok(Self(tz))
    }

    fn write(&self, _w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

impl Parameter for RelationshipType {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"RELTYPE")?;
        let r = match v {
            b"PARENT" => Self::Parent,
            b"CHILD" => Self::Child,
            b"SIBLING" => Self::Sibling,
            x => {
                if x.to_ascii_uppercase().starts_with(b"X-") {
                    Self::X(x.try_into()?)
                } else {
                    Self::Iana(x.try_into()?)
                }
            }
        };
        Ok(r)
    }

    fn write(&self, _w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

impl Parameter for AlarmTriggerRelationship {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"RELATED")?;
        let r = match v {
            b"START" => Self::Start,
            b"END" => Self::End,
            _ => {
                return Err(ParseError::Parameter {
                    expected: "START or END".into(),
                    received: std::str::from_utf8(v).ok().map(|s| s.into()),
                });
            }
        };
        Ok(r)
    }

    fn write(&self, _w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

impl Parameter for RecurrenceIdentifierRange {
    fn parse(b: Bytes) -> ParseResult<Self> {
        let (n, v) = split_once(&b, b'=')?;
        match_name(n, b"RANGE")?;
        match v {
            b"THISANDFUTURE" => Ok(Self::ThisAndFuture),
            _ => Err(ParseError::Parameter {
                expected: "THISANDFUTURE".into(),
                received: std::str::from_utf8(v).ok().map(|s| s.into()),
            }),
        }
    }

    fn write(&self, _w: impl Write) -> ParseResult<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::*;

    fn b(s: &'static [u8]) -> Bytes {
        Bytes::from_static(s)
    }

    // DataTypes

    #[test]
    fn data_types_ok() {
        let v = DataTypes::parse(b(b"VALUE=DATE-TIME")).unwrap();
        assert!(matches!(v, DataTypes::DateTime));
    }

    #[test]
    fn data_types_wrong_name() {
        assert!(DataTypes::parse(b(b"TYPE=DATE-TIME")).is_err());
    }

    // Altrep

    #[test]
    fn altrep_ok() {
        let v = Altrep::parse(b(b"ALTREP=\"http://example.com/cal\""));
        assert!(v.is_ok());
    }

    #[test]
    fn altrep_wrong_name() {
        assert!(Altrep::parse(b(b"Meow=\"http://example.com/cal\"")).is_err());
    }

    // CommonName

    #[test]
    fn common_name_ok() {
        let v = CommonName::parse(b(b"CN=\"John Smith\""));
        assert!(v.is_ok());
    }

    #[test]
    fn common_name_wrong_name() {
        assert!(CommonName::parse(b(b"Meow=\"John Smith\"")).is_err());
    }

    // Delegators

    #[test]
    fn delegators_ok() {
        let v = Delegators::parse(b(
            b"DELEGATED-FROM=\"mailto:a@example.com\",\"mailto:b@example.com\"",
        ));
        assert!(v.is_ok());
    }

    #[test]
    fn delegators_wrong_name() {
        assert!(
            Delegators::parse(b(b"DELEGATED-TO=\"mailto:a@example.com\""))
                .is_err()
        );
    }

    // Delegatees

    #[test]
    fn delegatees_ok() {
        let v =
            Delegatees::parse(b(b"DELEGATED-TO=\"mailto:jdoe@example.com\""));
        assert!(v.is_ok());
    }

    #[test]
    fn delegatees_wrong_name() {
        assert!(
            Delegatees::parse(b(b"DELEGATED-FROM=\"mailto:jdoe@example.com\""))
                .is_err()
        );
    }

    // DirectoryEntryReference

    #[test]
    fn dir_ok() {
        let v = DirectoryEntryReference::parse(b(
            b"DIR=\"http://example.com/dir\"",
        ));
        assert!(v.is_ok());
    }

    #[test]
    fn dir_wrong_name() {
        assert!(
            DirectoryEntryReference::parse(b(
                b"ALTREP=\"http://example.com/dir\""
            ))
            .is_err()
        );
    }

    // Encoding

    #[test]
    fn encoding_ok() {
        let v = Encoding::parse(b(b"ENCODING=BASE64")).unwrap();
        assert!(matches!(v, Encoding::Base64));
    }

    #[test]
    fn encoding_invalid_value() {
        assert!(Encoding::parse(b(b"ENCODING=QUOTED-PRINTABLE")).is_err());
    }

    // Fmttype

    #[test]
    fn fmttype_ok() {
        let v = Fmttype::parse(b(b"FMTTYPE=application/msword"));
        assert!(v.is_ok());
    }

    #[test]
    fn fmttype_no_slash() {
        assert!(Fmttype::parse(b(b"FMTTYPE=application")).is_err());
    }

    // Fbtype

    #[test]
    fn fbtype_ok() {
        let v = Fbtype::parse(b(b"FBTYPE=BUSY-UNAVAILABLE")).unwrap();
        assert!(matches!(v, Fbtype::BusyUnavailable));
    }

    // Language

    #[test]
    fn language_ok() {
        let v = Language::parse(b(b"LANGUAGE=en-US"));
        assert!(v.is_ok());
    }

    #[test]
    fn language_invalid_tag() {
        assert!(Language::parse(b(b"LANGUAGE=!!!")).is_err());
    }

    // Member

    #[test]
    fn member_ok() {
        let v = Member::parse(b(b"MEMBER=\"mailto:jsmith@example.com\""));
        assert!(v.is_ok());
    }

    #[test]
    fn member_unquoted() {
        assert!(Member::parse(b(b"MEMBER=mailto:jsmith@example.com")).is_err());
    }

    // CalendarUserType

    #[test]
    fn cutype_ok() {
        let v = CalendarUserType::parse(b(b"CUTYPE=ROOM")).unwrap();
        assert!(matches!(v, CalendarUserType::Room));
    }

    // ParticipationRole

    #[test]
    fn role_ok() {
        let v = ParticipationRole::parse(b(b"ROLE=CHAIR")).unwrap();
        assert!(matches!(v, ParticipationRole::Chair));
    }

    // PartStatEvent

    #[test]
    fn partstat_event_ok() {
        let v = PartStatEvent::parse(b(b"PARTSTAT=DECLINED")).unwrap();
        assert!(matches!(v, PartStatEvent::Declined));
    }

    // PartStatTodo

    #[test]
    fn partstat_todo_ok() {
        let v = PartStatTodo::parse(b(b"PARTSTAT=IN-PROCESS")).unwrap();
        assert!(matches!(v, PartStatTodo::InProcess));
    }

    // PartStatJournal

    #[test]
    fn partstat_journal_ok() {
        let v = PartStatJournal::parse(b(b"PARTSTAT=ACCEPTED")).unwrap();
        assert!(matches!(v, PartStatJournal::Accepted));
    }

    // Rsvp

    #[test]
    fn rsvp_ok() {
        let v = Rsvp::parse(b(b"RSVP=TRUE")).unwrap();
        assert!(*v.0);
    }

    #[test]
    fn rsvp_invalid_value() {
        assert!(Rsvp::parse(b(b"RSVP=MAYBE")).is_err());
    }

    // SentBy

    #[test]
    fn sent_by_ok() {
        let v = SentBy::parse(b(b"SENT-BY=\"mailto:proxy@example.com\""));
        assert!(v.is_ok());
    }

    #[test]
    fn sent_by_unquoted() {
        assert!(SentBy::parse(b(b"SENT-BY=mailto:proxy@example.com")).is_err());
    }

    // TimeZoneIdentifier

    #[test]
    fn tzid_ok() {
        let v = TimeZoneIdentifier::parse(b(b"TZID=America/New_York"));
        assert!(v.is_ok());
    }

    #[test]
    fn tzid_unknown_timezone() {
        assert!(TimeZoneIdentifier::parse(b(b"TZID=Not/A/Timezone")).is_err());
    }

    // RelationshipType

    #[test]
    fn reltype_ok() {
        let v = RelationshipType::parse(b(b"RELTYPE=SIBLING")).unwrap();
        assert!(matches!(v, RelationshipType::Sibling));
    }

    // AlarmTriggerRelationship

    #[test]
    fn alarm_related_ok() {
        let v = AlarmTriggerRelationship::parse(b(b"RELATED=END")).unwrap();
        assert!(matches!(v, AlarmTriggerRelationship::End));
    }

    #[test]
    fn alarm_related_invalid_value() {
        assert!(AlarmTriggerRelationship::parse(b(b"RELATED=MIDDLE")).is_err());
    }

    // RecurrenceIdentifierRange

    #[test]
    fn range_ok() {
        let v = RecurrenceIdentifierRange::parse(b(b"RANGE=THISANDFUTURE"))
            .unwrap();
        assert!(matches!(v, RecurrenceIdentifierRange::ThisAndFuture));
    }

    #[test]
    fn range_deprecated_value() {
        assert!(
            RecurrenceIdentifierRange::parse(b(b"RANGE=THISANDPRIOR")).is_err()
        );
    }

    // x-name and iana-token fallthrough

    #[test]
    fn data_types_xname() {
        let v = DataTypes::parse(b(b"VALUE=X-VENDOR-MYTYPE")).unwrap();
        assert!(matches!(v, DataTypes::XName(_)));
    }

    #[test]
    fn data_types_xname_lowercase_prefix() {
        // Detection is case-insensitive: "x-" must also route to XName
        let v = DataTypes::parse(b(b"VALUE=x-vendor-mytype")).unwrap();
        assert!(matches!(v, DataTypes::XName(_)));
    }

    #[test]
    fn data_types_iana() {
        let v = DataTypes::parse(b(b"VALUE=MY-CUSTOM-TYPE")).unwrap();
        assert!(matches!(v, DataTypes::Iana(_)));
    }

    #[test]
    fn fbtype_xname() {
        let v = Fbtype::parse(b(b"FBTYPE=X-VENDOR-BUSY")).unwrap();
        assert!(matches!(v, Fbtype::X(_)));
    }

    #[test]
    fn fbtype_xname_lowercase_prefix() {
        let v = Fbtype::parse(b(b"FBTYPE=x-vendor-busy")).unwrap();
        assert!(matches!(v, Fbtype::X(_)));
    }

    #[test]
    fn fbtype_iana() {
        let v = Fbtype::parse(b(b"FBTYPE=BUSY-CONDITIONAL")).unwrap();
        assert!(matches!(v, Fbtype::Iana(_)));
    }

    #[test]
    fn cutype_xname() {
        let v = CalendarUserType::parse(b(b"CUTYPE=X-DEVICE")).unwrap();
        assert!(matches!(v, CalendarUserType::X(_)));
    }

    #[test]
    fn cutype_xname_lowercase_prefix() {
        let v = CalendarUserType::parse(b(b"CUTYPE=x-device")).unwrap();
        assert!(matches!(v, CalendarUserType::X(_)));
    }

    #[test]
    fn cutype_iana() {
        let v = CalendarUserType::parse(b(b"CUTYPE=ORG-UNIT")).unwrap();
        assert!(matches!(v, CalendarUserType::Iana(_)));
    }

    #[test]
    fn role_xname() {
        let v = ParticipationRole::parse(b(b"ROLE=X-MODERATOR")).unwrap();
        assert!(matches!(v, ParticipationRole::X(_)));
    }

    #[test]
    fn role_xname_lowercase_prefix() {
        let v = ParticipationRole::parse(b(b"ROLE=x-moderator")).unwrap();
        assert!(matches!(v, ParticipationRole::X(_)));
    }

    #[test]
    fn role_iana() {
        let v = ParticipationRole::parse(b(b"ROLE=PRESENTER")).unwrap();
        assert!(matches!(v, ParticipationRole::Iana(_)));
    }

    #[test]
    fn partstat_event_xname() {
        let v = PartStatEvent::parse(b(b"PARTSTAT=X-PENDING")).unwrap();
        assert!(matches!(v, PartStatEvent::X(_)));
    }

    #[test]
    fn partstat_event_xname_lowercase_prefix() {
        let v = PartStatEvent::parse(b(b"PARTSTAT=x-pending")).unwrap();
        assert!(matches!(v, PartStatEvent::X(_)));
    }

    #[test]
    fn partstat_event_iana() {
        let v = PartStatEvent::parse(b(b"PARTSTAT=CONFIRMED")).unwrap();
        assert!(matches!(v, PartStatEvent::Iana(_)));
    }

    #[test]
    fn partstat_todo_xname() {
        let v = PartStatTodo::parse(b(b"PARTSTAT=X-BLOCKED")).unwrap();
        assert!(matches!(v, PartStatTodo::X(_)));
    }

    #[test]
    fn partstat_todo_xname_lowercase_prefix() {
        let v = PartStatTodo::parse(b(b"PARTSTAT=x-blocked")).unwrap();
        assert!(matches!(v, PartStatTodo::X(_)));
    }

    #[test]
    fn partstat_todo_iana() {
        let v = PartStatTodo::parse(b(b"PARTSTAT=CONFIRMED")).unwrap();
        assert!(matches!(v, PartStatTodo::Iana(_)));
    }

    #[test]
    fn partstat_journal_xname() {
        let v = PartStatJournal::parse(b(b"PARTSTAT=X-DRAFT")).unwrap();
        assert!(matches!(v, PartStatJournal::X(_)));
    }

    #[test]
    fn partstat_journal_xname_lowercase_prefix() {
        let v = PartStatJournal::parse(b(b"PARTSTAT=x-draft")).unwrap();
        assert!(matches!(v, PartStatJournal::X(_)));
    }

    #[test]
    fn partstat_journal_iana() {
        let v = PartStatJournal::parse(b(b"PARTSTAT=CONFIRMED")).unwrap();
        assert!(matches!(v, PartStatJournal::Iana(_)));
    }

    #[test]
    fn reltype_xname() {
        let v = RelationshipType::parse(b(b"RELTYPE=X-DEPENDS-ON")).unwrap();
        assert!(matches!(v, RelationshipType::X(_)));
    }

    #[test]
    fn reltype_xname_lowercase_prefix() {
        let v = RelationshipType::parse(b(b"RELTYPE=x-depends-on")).unwrap();
        assert!(matches!(v, RelationshipType::X(_)));
    }

    #[test]
    fn reltype_iana() {
        let v = RelationshipType::parse(b(b"RELTYPE=FINISHES")).unwrap();
        assert!(matches!(v, RelationshipType::Iana(_)));
    }
}
