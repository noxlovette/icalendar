use crate::values::{Boolean, CalendarUserAddress, Text};
use chrono_tz::Tz;
pub use recurrence::*;

/// Convenience wrapper around params
#[derive(Debug, Default)]
pub struct Params<T> {
    inner: Option<T>,
    iana: Vec<Text>,
    non_standard: Vec<Text>,
}

/// All Data types as params
pub enum DataTypes {
    Binary,
    Uri,
    Text,
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
    Bit8,
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
    Free,
    Busy,
    BusyUnavailable,
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
    #[default]
    Individual,
    Group,
    Resource,
    Room,
    Unknown,
    XType(String),
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
    Event(PartStatEvent),
    Todo(PartStatTodo),
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
    Chair,
    #[default]
    ReqParticipant,
    OptParticipant,
    NonParticipant,
    XName(String),
    IanaToken(String),
}

/// Participation statuses for a "VEVENT"
#[derive(Debug, Default)]
enum PartStatEvent {
    #[default]
    NeedsAction,
    Accepted,
    Declined,
    Tentative,
    Delegated,
    XName(String),
    IanaToken(String),
}

/// Participation statuses for a "VTODO"
#[derive(Debug, Default)]
enum PartStatTodo {
    #[default]
    NeedsAction,
    Accepted,
    Declined,
    Tentative,
    Delegated,
    Completed,
    InProcess,
    XName(String),
    IanaToken(String),
}

/// Participation statuses for a "VJOURNAL"
#[derive(Debug, Default)]
enum PartStatJournal {
    #[default]
    NeedsAction,
    Accepted,
    Declined,
    XName(String),
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
    #[default]
    Parent,
    Child,
    Sibling,
    XName(String),
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
pub enum Related {
    #[default]
    Start,
    End,
}

mod recurrence {
    use crate::values::{Date, DateTime};

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
    pub enum Range {
        ThisAndFuture,
    }

    /// The RFC 5545's helper
    #[derive(Debug, Clone)]
    pub enum DateOrDatetime {
        Date(Date),
        DateTime(DateTime),
    }
    /// Enforces 0 to 60
    #[derive(Debug, Clone)]
    pub struct Seconds(u8);
    /// 0 to 59
    #[derive(Debug, Clone)]
    pub struct Minutes(u8);
    /// 0 to 23
    #[derive(Debug, Clone)]
    pub struct Hour(u8);
    #[derive(Debug, Clone)]
    pub struct WeekNum(i8);

    /// 1 to 53, Ordinal of the week
    pub struct OrdWk(u8);
    #[derive(Debug, Clone)]
    pub struct WeekdayNum {
        ordinal: Option<i8>,
        weekday: Weekday,
    }
    /// 1 to 12
    #[derive(Debug, Clone)]
    pub struct MonthNum(u8);
    /// 1 to 31, Ordinal of month day
    pub struct OrdMoDay(u8);
    #[derive(Debug, Clone)]
    pub struct MonthDayNum(i8);
    /// 1 to 366
    pub struct OrdYrDay(u16);
    #[derive(Debug, Clone)]
    pub struct YearDayNum(i16);
    pub type SetPosDay = YearDayNum;

    /// Day of the week
    #[derive(Debug, Clone, Default)]
    pub enum Weekday {
        Su,
        #[default]
        Mo,
        Tu,
        We,
        Th,
        Fr,
        Sa,
    }

    /// This value type is a structured value consisting of a
    /// list of one or more recurrence grammar parts.  Each rule part is
    /// defined by a NAME=VALUE pair.  The rule parts are separated from
    /// each other by the SEMICOLON character.  The rule parts are not
    /// ordered in any particular sequence.  Individual rule parts MUST
    /// only be specified once.  Compliant applications MUST accept rule
    /// parts ordered in any sequence, but to ensure backward
    /// compatibility with applications that pre-date this revision of
    /// iCalendar the FREQ rule part MUST be the first rule part specified
    /// in a RECUR value.
    ///
    /// Recurrence rules may generate recurrence instances with an invalid
    /// date (e.g., February 30) or nonexistent local time (e.g., 1:30 AM
    /// on a day where the local time is moved forward by an hour at 1:00
    /// AM).  Such recurrence instances MUST be ignored and MUST NOT be
    /// counted as part of the recurrence set.
    ///
    /// Information, not contained in the rule, necessary to determine the
    /// various recurrence instance start time and dates are derived from
    /// the Start Time ("DTSTART") component attribute.  For example,
    /// "FREQ=YEARLY;BYMONTH=1" doesn't specify a specific day within the
    /// month or a time.  This information would be the same as what is
    /// specified for "DTSTART".
    ///
    /// [More](https://datatracker.ietf.org/doc/html/rfc5545#autoid-42)
    #[derive(Debug, Clone, Default)]
    pub struct Recur {
        /// The FREQ rule part identifies the type of recurrence rule. This
        /// rule part MUST be specified in the recurrence rule.  Valid values
        /// include SECONDLY, to specify repeating events based on an interval
        /// of a second or more; MINUTELY, to specify repeating events based
        /// on an interval of a minute or more; HOURLY, to specify repeating
        /// events based on an interval of an hour or more; DAILY, to specify
        /// repeating events based on an interval of a day or more; WEEKLY, to
        /// specify repeating events based on an interval of a week or more;
        /// MONTHLY, to specify repeating events based on an interval of a
        /// month or more; and YEARLY, to specify repeating events based on an
        /// interval of a year or more.
        freq: Frequency,
        /// The UNTIL rule part defines a DATE or DATE-TIME value that bounds
        /// the recurrence rule in an inclusive manner.  If the value
        /// specified by UNTIL is synchronized with the specified recurrence,
        /// this DATE or DATE-TIME becomes the last instance of the
        /// recurrence.  The value of the UNTIL rule part MUST have the same
        /// value type as the "DTSTART" property.  Furthermore, if the
        /// "DTSTART" property is specified as a date with local time, then
        /// the UNTIL rule part MUST also be specified as a date with local
        /// time.  If the "DTSTART" property is specified as a date with UTC
        /// time or a date with local time and time zone reference, then the
        /// UNTIL rule part MUST be specified as a date with UTC time.  In the
        /// case of the "STANDARD" and "DAYLIGHT" sub-components the UNTIL
        /// rule part MUST always be specified as a date with UTC time.  If
        /// specified as a DATE-TIME value, then it MUST be specified in a UTC
        /// time format.  If not present, and the COUNT rule part is also not
        /// present, the "RRULE" is considered to repeat forever.
        until: Option<DateOrDatetime>,
        /// The COUNT rule part defines the number of occurrences at which to
        /// range-bound the recurrence.  The "DTSTART" property value always
        /// counts as the first occurrence.
        count: Option<i32>,
        /// The INTERVAL rule part contains a positive integer representing at
        /// which intervals the recurrence rule repeats.  The default value is
        /// "1", meaning every second for a SECONDLY rule, every minute for a
        /// MINUTELY rule, every hour for an HOURLY rule, every day for a
        /// DAILY rule, every week for a WEEKLY rule, every month for a
        /// MONTHLY rule, and every year for a YEARLY rule.  For example,
        /// within a DAILY rule, a value of "8" means every eight days.
        interval: Option<i32>,
        /// The BYSECOND rule part specifies a COMMA-separated list of seconds
        /// a minute.  Valid values are 0 to 60.  The BYMINUTE rule
        /// specifies a COMMA-separated list of minutes within an hour.
        /// values are 0 to 59.  The BYHOUR rule part specifies a COMMA-
        /// list of hours of the day.  Valid values are 0 to 23.
        /// BYSECOND, BYMINUTE and BYHOUR rule parts MUST NOT be specified
        /// the associated "DTSTART" property has a DATE value type.
        /// rule parts MUST be ignored in RECUR value that violate the
        /// requirement (e.g., generated by applications that pre-date
        /// revision of iCalendar).
        by_second: Vec<Seconds>,
        by_minute: Vec<Minutes>,
        by_hour: Vec<Hour>,
        /// The BYDAY rule part specifies a COMMA-separated list of days of
        /// week; SU indicates Sunday; MO indicates Monday; TU indicates
        /// Tuesday; WE indicates Wednesday; TH indicates Thursday; FR
        /// Friday; and SA indicates Saturday.
        ///
        /// Each BYDAY value can also be preceded by a positive (+n) or
        /// negative (-n) integer.  If present, this indicates the nth
        /// occurrence of a specific day within the MONTHLY or YEARLY "RRULE".
        ///
        /// For example, within a MONTHLY rule, +1MO (or simply 1MO)
        /// represents the first Monday within the month, whereas -1MO
        /// represents the last Monday of the month.  The numeric value in a
        /// BYDAY rule part with the FREQ rule part set to YEARLY corresponds
        /// to an offset within the month when the BYMONTH rule part is
        /// present, and corresponds to an offset within the year when the
        /// BYWEEKNO or BYMONTH rule parts are present.  If an integer
        /// modifier is not present, it means all days of this type within the
        /// specified frequency.  For example, within a MONTHLY rule, MO
        /// represents all Mondays within the month.  The BYDAY rule part MUST
        /// NOT be specified with a numeric value when the FREQ rule part is
        /// not set to MONTHLY or YEARLY.  Furthermore, the BYDAY rule part
        /// MUST NOT be specified with a numeric value with the FREQ rule part
        /// set to YEARLY when the BYWEEKNO rule part is specified.
        by_day: Vec<WeekdayNum>,
        /// The BYMONTHDAY rule part specifies a COMMA-separated list of days
        /// of the month.  Valid values are 1 to 31 or -31 to -1.  For
        /// example, -10 represents the tenth to the last day of the month.
        /// The BYMONTHDAY rule part MUST NOT be specified when the FREQ rule
        /// part is set to WEEKLY.
        by_month_day: Vec<MonthDayNum>,
        /// The BYYEARDAY rule part specifies a COMMA-separated list of days
        /// of the year.  Valid values are 1 to 366 or -366 to -1.  For
        /// example, -1 represents the last day of the year (December 31st)
        /// and -306 represents the 306th to the last day of the year (March
        /// 1st).  The BYYEARDAY rule part MUST NOT be specified when the FREQ
        /// rule part is set to DAILY, WEEKLY, or MONTHLY.
        by_year_day: Vec<YearDayNum>,
        /// The BYWEEKNO rule part specifies a COMMA-separated list of
        /// ordinals specifying weeks of the year.  Valid values are 1 to 53
        /// or -53 to -1.  This corresponds to weeks according to week
        /// numbering as defined in [ISO.8601.2004].  A week is defined as a
        /// seven day period, starting on the day of the week defined to be
        /// the week start (see WKST).  Week number one of the calendar year
        /// is the first week that contains at least four (4) days in that
        /// calendar year.  This rule part MUST NOT be used when the FREQ rule
        /// part is set to anything other than YEARLY.  For example, 3
        /// represents the third week of the year.
        ///
        /// > Note: Assuming a Monday week start, week 53 can only occur when
        /// > Thursday is January 1 or if it is a leap year and Wednesday is
        /// > January 1.
        by_week_no: Vec<WeekNum>,
        /// The BYMONTH rule part specifies a COMMA-separated list of months
        /// of the year.  Valid values are 1 to 12.
        by_month: Vec<MonthNum>,

        /// The BYSETPOS rule part specifies a COMMA-separated list of values
        /// that corresponds to the nth occurrence within the set of
        /// recurrence instances specified by the rule.  BYSETPOS operates on
        /// a set of recurrence instances in one interval of the recurrence
        /// rule.  For example, in a WEEKLY rule, the interval would be one
        /// week A set of recurrence instances starts at the beginning of the
        /// interval defined by the FREQ rule part.  Valid values are 1 to 366
        /// or -366 to -1.  It MUST only be used in conjunction with another
        /// BYxxx rule part.  For example "the last work day of the month"
        /// could be represented as:
        ///
        /// FREQ=MONTHLY;BYDAY=MO,TU,WE,TH,FR;BYSETPOS=-1
        by_set_pos: Vec<SetPosDay>,
        /// The WKST rule part specifies the day on which the workweek starts.
        /// Valid values are MO, TU, WE, TH, FR, SA, and SU.  This is
        /// significant when a WEEKLY "RRULE" has an interval greater
        /// than 1, and a BYDAY rule part is specified. This is also
        /// significant when in a YEARLY "RRULE" when a BYWEEKNO rule
        /// part is specified. The default value is MO.
        wkst: Option<Weekday>,
    }

    /// The FREQ rule part identifies the type of recurrence rule. This
    /// rule part MUST be specified in the recurrence rule.  Valid values
    /// include SECONDLY, to specify repeating events based on an interval
    /// of a second or more; MINUTELY, to specify repeating events based
    /// on an interval of a minute or more; HOURLY, to specify repeating
    /// events based on an interval of an hour or more; DAILY, to specify
    /// repeating events based on an interval of a day or more; WEEKLY, to
    /// specify repeating events based on an interval of a week or more;
    /// MONTHLY, to specify repeating events based on an interval of a
    /// month or more; and YEARLY, to specify repeating events based on an
    /// interval of a year or more.
    #[derive(Debug, Clone, Default)]
    pub enum Frequency {
        Secondly,
        Minutely,
        Hourly,
        /// Every N days.
        Daily,
        /// Every N weeks.
        #[default]
        Weekly,
        /// Every N months.
        Monthly,
        /// Every N years.
        Yearly,
    }
}
