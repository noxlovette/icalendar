use std::ops::Deref;

use crate::{
    ParseError,
    internal::{split_once, strip_quoted_string},
    params::TimeZoneIdentifier,
};
use base64::alphabet::Alphabet;
use bytes::Bytes;
use chrono::{
    DateTime as ChronoDateTime, Duration as ChronoDuration, FixedOffset,
    NaiveDate, NaiveTime, Utc,
};
pub use recurrence::Recur;
use url::Url;

/// The RFC 5545's helper
#[derive(Debug, Clone)]
pub enum DateOrDatetime {
    /// A calendar date without a time component.
    Date(Date),
    /// A precise calendar date and time of day.
    DateTime(DateTime),
}

/// Convenience union of [`Date`], [`DateTime`], and [`Period`] used by
/// properties that accept any of those three value types (e.g., `FREEBUSY`).
pub enum DateTimePeriod {
    /// A calendar date without a time component.
    Date(Date),
    /// A precise calendar date and time of day.
    DateTime(DateTime),
    /// A span of time defined by start/end or start/duration.
    Period(Period),
}

/// Convenience union of [`Duration`] and [`DateTime`] used by properties
/// that accept either value type (e.g., `TRIGGER`).
pub enum DateTimeDuration {
    /// A positive span of time.
    Duration(Duration),
    /// A precise calendar date and time of day.
    DateTime(DateTime),
}

/// If the property permits, multiple "duration" values are
/// specified by a COMMA-separated list of values.  The format is
/// based on the [ISO.8601.2004] complete representation basic format
/// with designators for the duration of time.  The format can
/// represent nominal durations (weeks and days) and accurate
/// durations (hours, minutes, and seconds).  Note that unlike
/// [ISO.8601.2004], this value type doesn't support the "Y" and "M"
/// designators to specify durations in terms of years and months.
///
/// The duration of a week or a day depends on its position in the
/// calendar.  In the case of discontinuities in the time scale, such
/// as the change from standard time to daylight time and back, the
/// computation of the exact duration requires the subtraction or
/// addition of the change of duration of the discontinuity.  Leap
/// seconds MUST NOT be considered when computing an exact duration.
/// When computing an exact duration, the greatest order time
/// components MUST be added first, that is, the number of days MUST
/// be added first, followed by the number of hours, number of
/// minutes, and number of seconds.
/// Negative durations are typically used to schedule an alarm to
/// trigger before an associated time (see Section 3.8.6.3).
///
/// Example:  A duration of 15 days, 5 hours, and 20 seconds would be:
///
/// > P15DT5H0M20S
///
/// A duration of 7 weeks would be:
///
/// > P7W
///
/// [Section 3.3.6](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.6)
pub type Duration = ChronoDuration;

/// If the property permits, multiple "DATE-TIME" values
/// are specified as a COMMA-separated list of values.  No additional
/// content value encoding (i.e., BACKSLASH character encoding, see
/// Section 3.3.11) is defined for this value type.
///
/// The "DATE-TIME" value type is used to identify values that contain
/// a precise calendar date and time of day.  The format is based on
/// the [ISO.8601.2004] complete representation, basic format for a
/// calendar date and time of day.  The text format is a concatenation
/// of the "date", followed by the LATIN CAPITAL LETTER T character,
/// the time designator, followed by the "time" format.
///
/// The "DATE-TIME" value type expresses time values in three forms:
/// The form of date and time with UTC offset MUST NOT be used.  For
/// example, the following is not valid for a DATE-TIME value:
///
/// > 19980119T230000-0800       ;Invalid time format
///
/// ## FORM #1: DATE WITH LOCAL TIME
///
/// The date with local time form is simply a DATE-TIME value that
/// does not contain the UTC designator nor does it reference a time
/// zone.  For example, the following represents January 18, 1998, at
/// 11 PM:
///
/// > 19980118T230000
///
/// DATE-TIME values of this type are said to be "floating" and are
/// not bound to any time zone in particular.  They are used to
/// represent the same hour, minute, and second value regardless of
/// which time zone is currently being observed.  For example, an
/// event can be defined that indicates that an individual will be
/// busy from 11:00 AM to 1:00 PM every day, no matter which time zone
/// the person is in.  In these cases, a local time can be specified.
/// The recipient of an iCalendar object with a property value
/// consisting of a local time, without any relative time zone
/// information, SHOULD interpret the value as being fixed to whatever
/// time zone the "ATTENDEE" is in at any given moment.  This means
/// that two "Attendees", in different time zones, receiving the same
/// event definition as a floating time, may be participating in the
/// event at different actual times.  Floating time SHOULD only be
/// used where that is the reasonable behavior.
///
/// In most cases, a fixed time is desired.  To properly communicate a
/// fixed time in a property value, either UTC time or local time with
/// time zone reference MUST be specified.
///
/// The use of local time in a DATE-TIME value without the "TZID"
/// property parameter is to be interpreted as floating time,
/// regardless of the existence of "VTIMEZONE" calendar components in
/// the iCalendar object.
///
/// ## FORM #2: DATE WITH UTC TIME
///
/// The date with UTC time, or absolute time, is identified by a LATIN
/// CAPITAL LETTER Z suffix character, the UTC designator, appended to
/// the time value.  For example, the following represents January 19,
/// 1998, at 0700 UTC:
///
/// > 19980119T070000Z
///
/// The "TZID" property parameter MUST NOT be applied to DATE-TIME
/// properties whose time values are specified in UTC.
/// FORM #3: DATE WITH LOCAL TIME AND TIME ZONE REFERENCE
/// The date and local time with reference to time zone information is
/// identified by the use the "TZID" property parameter to reference
/// the appropriate time zone definition.  "TZID" is discussed in
/// detail in Section 3.2.19.  For example, the following represents
/// 2:00 A.M. in New York on January 19, 1998:
///
/// > TZID=America/New_York:19980119T020000
///
/// If, based on the definition of the referenced time zone, the local
/// time described occurs more than once (when changing from daylight
/// to standard time), the DATE-TIME value refers to the first
/// occurrence of the referenced time.  Thus, TZID=America/
/// New_York:20071104T013000 indicates November 4, 2007 at 1:30 A.M.
/// EDT (UTC-04:00).  If the local time described does not occur (when
/// changing from standard to daylight time), the DATE-TIME value is
/// interpreted using the UTC offset before the gap in local times.
/// Thus, TZID=America/New_York:20070311T023000 indicates March 11,
/// 2007 at 3:30 A.M. EDT (UTC-04:00), one hour after 1:30 A.M. EST
/// (UTC-05:00).
///
/// A time value MUST only specify the second 60 when specifying a
/// positive leap second.  For example:
///
/// > 19970630T235960Z
///
/// Implementations that do not support leap seconds SHOULD interpret
/// the second 60 as equivalent to the second 59.
///
/// Example:  The following represents July 14, 1997, at 1:30 PM in New
/// York City in each of the three time formats, using the "DTSTART"
/// property.
///
/// > DTSTART:19970714T133000                   ; Local time
/// > DTSTART:19970714T173000Z                  ; UTC time
/// > DTSTART;TZID=America/New_York:19970714T133000 ; Local time and time ; zone
/// > reference
///
/// [Section 3.3.5](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.5)
#[derive(Debug, Clone)]
pub enum DateTime {
    /// Local (floating) time — not bound to any time zone.
    Floating(NaiveDate),
    /// Absolute UTC time, identified by the `Z` suffix.
    Utc(ChronoDateTime<Utc>),
    /// Local time anchored to a specific time zone via a TZID reference.
    Timezone {
        /// The local date.
        dt: Date,
        /// The VTIMEZONE identifier that gives the offset context.
        tzid: TimeZoneIdentifier,
    },
}

/// If the property permits, multiple "date" values are
/// specified as a COMMA-separated list of values.  The format for the
/// value type is based on the [ISO.8601.2004] complete
/// representation, basic format for a calendar date.  The textual
/// format specifies a four-digit year, two-digit month, and two-digit
/// day of the month.  There are no separator characters between the
/// year, month, and day component text.
///
/// Example:  The following represents July 14, 1997:
///
/// > 19970714
///
/// [Section 3.3.4](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.4)
pub type Date = NaiveDate;
/// The PLUS SIGN character MUST be specified for positive
/// UTC offsets (i.e., ahead of UTC).  The HYPHEN-MINUS character MUST
/// be specified for negative UTC offsets (i.e., behind of UTC).  The
/// value of "-0000" and "-000000" are not allowed.  The time-second,
/// if present, MUST NOT be 60; if absent, it defaults to zero.
/// No additional content value encoding (i.e., BACKSLASH character
/// encoding, see Section 3.3.11) is defined for this value type.
/// Example:  The following UTC offsets are given for standard time for
/// New York (five hours behind UTC) and Geneva (one hour ahead of
/// UTC):
///
/// > -0500
/// >
/// > +0100
///
/// [Section 3.3.14](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.14)
pub type UtcOffset = FixedOffset;

/// If the property permits, multiple "period" values are
/// specified by a COMMA-separated list of values.  There are two
/// forms of a period of time.  First, a period of time is identified
/// by its start and its end.  This format is based on the
/// [ISO.8601.2004](https://datatracker.ietf.org/doc/html/rfc5545#ref-ISO.8601.2004) complete representation, basic format for "DATE-
/// TIME" start of the period, followed by a SOLIDUS character
/// followed by the "DATE-TIME" of the end of the period.  The start
/// of the period MUST be before the end of the period.  Second, a
/// period of time can also be defined by a start and a positive
/// duration of time.  The format is based on the [ISO.8601.2004](https://datatracker.ietf.org/doc/html/rfc5545#ref-ISO.8601.2004)
/// complete representation, basic format for the "DATE-TIME" start of
/// the period, followed by a SOLIDUS character, followed by the
/// [ISO.8601.2004](https://datatracker.ietf.org/doc/html/rfc5545#ref-ISO.8601.2004) basic format for "DURATION" of the period.
///
/// [Section 3.3.9](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.9)
#[derive(Debug)]
pub enum Period {
    /// Period defined by an explicit start and end date-time.
    StartEnd {
        /// Inclusive start of the period.
        start: DateTime,
        /// Exclusive end of the period; MUST be after `start`.
        end: DateTime,
    },
    /// Period defined by a start date-time and a positive duration.
    Duration {
        /// Start of the period.
        start: DateTime,
        /// Length of the period.
        duration: Duration,
    },
}

/// If the property permits, multiple "time" values are
/// specified by a COMMA-separated list of values.  No additional
/// content value encoding (i.e., BACKSLASH character encoding, see
/// Section 3.3.11) is defined for this value type.
///
/// The "TIME" value type is used to identify values that contain a
/// time of day.  The format is based on the [ISO.8601.2004] complete
/// representation, basic format for a time of day.  The text format
/// consists of a two-digit, 24-hour of the day (i.e., values 00-23),
/// two-digit minute in the hour (i.e., values 00-59), and two-digit
/// seconds in the minute (i.e., values 00-60).  The seconds value of
/// 60 MUST only be used to account for positive "leap" seconds.
/// Fractions of a second are not supported by this format.
///
/// ### In parallel to the "DATE-TIME" definition above, the "TIME" value
/// type expresses time values in three forms:
///
/// The form of time with UTC offset MUST NOT be used.  For example,
/// the following is not valid for a time value:
///
/// > 230000-0800        ;Invalid time format
///
/// ##   FORM #1 LOCAL TIME
///
/// The local time form is simply a time value that does not contain
/// the UTC designator nor does it reference a time zone.  For
/// example, 11:00 PM:
///
/// > 230000
///
/// Time values of this type are said to be "floating" and are not
/// bound to any time zone in particular.  They are used to represent
/// the same hour, minute, and second value regardless of which time
/// zone is currently being observed.  For example, an event can be
/// defined that indicates that an individual will be busy from 11:00
/// AM to 1:00 PM every day, no matter which time zone the person is
/// in.  In these cases, a local time can be specified.  The recipient
/// of an iCalendar object with a property value consisting of a local
/// time, without any relative time zone information, SHOULD interpret
/// the value as being fixed to whatever time zone the "ATTENDEE" is
/// in at any given moment.  This means that two "Attendees", may
/// participate in the same event at different UTC times; floating
/// time SHOULD only be used where that is reasonable behavior.
///
/// In most cases, a fixed time is desired.  To properly communicate
/// a fixed time in a property value, either UTC time or local time
/// with time zone reference MUST be specified.
///
/// The use of local time in a TIME value without the "TZID"
/// property parameter is to be interpreted as floating time,
/// regardless of the existence of "VTIMEZONE" calendar components
/// in the iCalendar object.
///
/// ## FORM #2: UTC TIME
///
/// UTC time, or absolute time, is identified by a LATIN CAPITAL
/// LETTER Z suffix character, the UTC designator, appended to the
/// time value.  For example, the following represents 07:00 AM UTC:
///
/// > 070000Z
///
/// The "TZID" property parameter MUST NOT be applied to TIME
/// properties whose time values are specified in UTC.
///
/// ## FORM #3: LOCAL TIME AND TIME ZONE REFERENCE
///
/// The local time with reference to time zone information form is
/// identified by the use the "TZID" property parameter to reference
/// the appropriate time zone definition.  "TZID" is discussed in
/// detail in Section 3.2.19.
///
/// Example:  The following represents 8:30 AM in New York in winter,
/// five hours behind UTC, in each of the three formats:
///
/// > 083000
/// >
/// > 133000Z
/// >
/// > TZID=America/New_York:083000
///
/// [Section 3.3.12](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.12)
pub enum Time {
    /// Local (floating) time — not bound to any time zone.
    Floating(NaiveTime),
    /// Local time anchored to a specific time zone via a TZID reference.
    Zoned {
        /// The clock time.
        time: NaiveTime,
        /// The VTIMEZONE identifier that gives the offset context.
        tzid: TimeZoneIdentifier,
    },
}

/// Description:  Property values with this value type MUST also include
/// the inline encoding parameter sequence of ";ENCODING=BASE64".
/// That is, all inline binary data MUST first be character encoded
/// using the "BASE64" encoding method defined in [RFC2045](https://datatracker.ietf.org/doc/html/rfc2045).  No
/// additional content value encoding (i.e., BACKSLASH character
/// encoding, see Section [3.3.11](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11)) is defined for this value type.
/// Example: The following is an example of a "BASE64" encoded binary
/// value data:
/// > ATTACH;FMTTYPE=image/vnd.microsoft.icon;ENCODING=BASE64;VALUE
/// > =BINARY:AAABAAEAEBAQAAEABAAoAQAAFgAAACgAAAAQAAAAIAAAAAEABAAA
/// > AAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAAAAgIAAAICAgADAwMAA////AAAA
/// > AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
/// > AAAAAAAAAAAAAAAAAAAAAAMwAAAAAAABNEMQAAAAAAAkQgAAAAAAJEREQgAA
/// > ACECQ0QgEgAAQxQzM0E0AABERCRCREQAADRDJEJEQwAAAhA0QwEQAAAAAERE
/// > AAAAAAAAREQAAAAAAAAkQgAAAAAAAAMgAAAAAAAAAAAAAAAAAAAAAAAAAAAA
/// > AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
/// > AAAAAAAAAAAA
///
/// [Section 3.3.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.1)
pub type Binary = Alphabet;

/// These values are case-insensitive text.  No additional
/// content value encoding (i.e., BACKSLASH character encoding, see
/// [Section 3.3.11](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11))
/// is defined for this value type.
///
/// [Section 3.3.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.2)
pub struct Boolean(bool);

/// If the property permits, multiple TEXT values are
/// specified by a COMMA-separated list of values.
///
/// The language in which the text is represented can be controlled by
/// the "LANGUAGE" property parameter.
///
/// An intentional formatted text line break MUST only be included in
/// a "TEXT" property value by representing the line break with the
/// character sequence of BACKSLASH, followed by a LATIN SMALL LETTER
/// N or a LATIN CAPITAL LETTER N, that is "\n" or "\N".
///
/// The "TEXT" property values may also contain special characters
/// that are used to signify delimiters, such as a COMMA character for
/// lists of values or a SEMICOLON character for structured values.
/// In order to support the inclusion of these special characters in
/// "TEXT" property values, they MUST be escaped with a BACKSLASH
/// character.  A BACKSLASH character in a "TEXT" property value MUST
/// be escaped with another BACKSLASH character.  A COMMA character in
/// a "TEXT" property value MUST be escaped with a BACKSLASH
/// character.  A SEMICOLON character in a "TEXT" property value MUST
/// be escaped with a BACKSLASH character.  However, a COLON character
/// in a "TEXT" property value SHALL NOT be escaped with a BACKSLASH
/// character.
///
/// Example:  A multiple line value of:
///
/// > Project XYZ Final Review
/// >
/// > Conference Room - 3B
/// >
/// > Come Prepared.
///
/// would be represented as:
///
/// > Project XYZ Final Review\nConference Room - 3B\nCome Prepared.
///
/// [Section 3.3.11](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11)
#[derive(Debug)]
pub struct Text(String);

/// This value type might be used to reference binary
/// information, for values that are large, or otherwise undesirable
/// to include directly in the iCalendar object.
/// Property values with this value type MUST follow the generic URI
/// syntax defined in [RFC3986](https://datatracker.ietf.org/doc/html/rfc3986).
///
/// When a property parameter value is a URI value type, the URI MUST
/// be specified as a quoted-string value.
///
/// No additional content value encoding (i.e., BACKSLASH character
/// encoding, see [Section 3.3.11](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.11))
/// is defined for this value type.
///
/// Example:  The following is a URI for a network file:
/// > http://example.com/my-report.txt
///
/// [Section 3.3.13](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.13)
#[derive(Debug)]
pub struct Uri(Url);

/// If the property permits, multiple "integer" values are
/// specified by a COMMA-separated list of values.  The valid range
/// for "integer" is -2147483648 to 2147483647.  If the sign is not
/// specified, then the value is assumed to be positive.
///
/// [Section 3.3.8](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.8)
pub type Integer = i32;
/// If the property permits, multiple "float" values are
/// specified by a COMMA-separated list of values.
///
/// [Section 3.3.7](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.7)
pub type Float = f64;
/// The value is a URI as defined by [RFC3986] or any other
/// IANA-registered form for a URI.  When used to address an Internet
/// email transport address for a calendar user, the value MUST be a
/// mailto URI, as defined by [RFC2368].  No additional content value
/// encoding (i.e., BACKSLASH character encoding, see Section 3.3.11)
/// is defined for this value type.
///
/// Example:
///
/// > mailto:jane_doe@example.com
///
/// [Section 3.3.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.3)
pub struct CalendarUserAddress(Uri);

mod recurrence {
    use super::DateOrDatetime;

    /// Enforces 0 to 60
    #[derive(Debug, Clone)]
    struct Seconds(u8);
    /// 0 to 59
    #[derive(Debug, Clone)]
    struct Minutes(u8);
    /// 0 to 23
    #[derive(Debug, Clone)]
    struct Hour(u8);
    #[derive(Debug, Clone)]
    struct WeekNum(i8);

    /// 1 to 53, Ordinal of the week
    struct OrdWk(u8);
    #[derive(Debug, Clone)]
    struct WeekdayNum {
        ordinal: Option<i8>,
        weekday: Weekday,
    }
    /// 1 to 12
    #[derive(Debug, Clone)]
    struct MonthNum(u8);
    /// 1 to 31, Ordinal of month day
    struct OrdMoDay(u8);
    #[derive(Debug, Clone)]
    struct MonthDayNum(i8);
    /// 1 to 366
    struct OrdYrDay(u16);
    #[derive(Debug, Clone)]
    struct YearDayNum(i16);
    type SetPosDay = YearDayNum;

    /// Day of the week
    #[derive(Debug, Clone, Default)]
    enum Weekday {
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
    enum Frequency {
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

/// [RFC 4288](https://datatracker.ietf.org/doc/html/rfc4288#section-4.2)
///
/// As used in this crate, we only validate that there is a type and subtype,
/// separated by a slash
pub struct MediaType {
    media_type: Text,
    subtype: Text,
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
impl TryFrom<&[u8]> for MediaType {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (t, s) = split_once(value, b'/')?;
        Ok(Self {
            media_type: t.try_into()?,
            subtype: s.try_into()?,
        })
    }
}

impl TryFrom<&[u8]> for Text {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(std::str::from_utf8(value)?.into())
    }
}

impl TryFrom<&[u8]> for Uri {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(url::Url::parse(str::from_utf8(
            strip_quoted_string(value)?,
        )?)?))
    }
}

impl TryFrom<&[u8]> for CalendarUserAddress {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let uri: Uri = value.try_into()?;
        if uri.scheme() != "mailto" {
            Err(ParseError::CalUserAddress)
        } else {
            Ok(Self(uri))
        }
    }
}

impl Deref for Uri {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Text {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Boolean {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&[u8]> for Boolean {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let r = match value {
            b"TRUE" => Self(true),
            b"FALSE" => Self(false),
            _ => return Err(ParseError::Boolean),
        };

        Ok(r)
    }
}
