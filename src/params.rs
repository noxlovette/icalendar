use crate::values::{Boolean, CalendarUserAddress, Text, Uri};
use chrono_tz::Tz;
pub use crate::values::Recur;

/// Convenience wrapper around params
#[derive(Debug, Default)]
pub struct Params<T: Default> {
    standard: T,
    iana: Vec<Text>,
    non_standard: Vec<Text>,
}

/// Convenience type that multiple components share
#[derive(Default)]
pub struct TextParams {
    altrep: Option<Altrep>,
    language: Option<Language>,
}

/// Convenience parameter bundle shared by date/time properties (`DTSTART`, `DTEND`, etc.).
#[derive(Default)]
pub struct TimeParams {
    vtype: Option<DataTypes>,
    tzid: Option<TimeZoneIdentifier>,
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
    XName(String),
    /// An IANA-registered type name.
    Iana(String),
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
pub struct Altrep(String);

/// This parameter can be specified on properties with a
/// CAL-ADDRESS value type.  The parameter specifies the common name
/// to be associated with the calendar user specified by the property.
/// The parameter value is text.  The parameter value can be used for
/// display text to be associated with the calendar address specified
/// by the property.
///
/// Example:
///
/// > ATTENDEE;CUTYPE=GROUP:mailto:ietf-calsch@example.org
///
/// [Section 3.2.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.2)
pub struct CommonName(String);

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
pub struct Delegators(String);

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
pub struct Delegatees(String);

/// This parameter specifies a reference to a directory entry associated with
/// the calendar user specified by the property.  The parameter value is a
/// URI.  The URI parameter value MUST be specified in a quoted-string.
///
/// Example:
///
/// > ORGANIZER;DIR="ldap://example.com:6666/o=ABC%20Industries,c=US???(cn=Jim%20Dolittle)":mailto:jimdo@example.com
///
/// [Section 3.2.6](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2.6)
pub struct DirectoryEntryReference(Uri);

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
pub struct Fmttype(String);

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
pub enum Fbtype {
    /// The interval is free for scheduling.
    Free,
    /// The interval is busy (one or more events scheduled).
    Busy,
    /// The interval is busy and cannot be scheduled.
    BusyUnavailable,
    /// The interval is tentatively busy.
    BusyTentative,
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
pub struct Language(langtag::LanguageBuf);

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
    /// A non-standard `X-` prefixed type.
    XType(String),
    /// An IANA-registered type.
    IanaToken(String),
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
    /// A non-standard `X-` prefixed role.
    XName(String),
    /// An IANA-registered role.
    IanaToken(String),
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
    /// A non-standard `X-` prefixed status.
    XName(String),
    /// An IANA-registered status.
    IanaToken(String),
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
    /// A non-standard `X-` prefixed status.
    XName(String),
    /// An IANA-registered status.
    IanaToken(String),
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
    /// A non-standard `X-` prefixed status.
    XName(String),
    /// An IANA-registered status.
    IanaToken(String),
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
    /// A non-standard `X-` prefixed relationship type.
    XName(String),
    /// An IANA-registered relationship type.
    IanaToken(String),
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

/// Shorthand alias for [`RecurrenceIdentifierRange`], used by [`crate::RecurrenceSet`].
pub type Range = RecurrenceIdentifierRange;
