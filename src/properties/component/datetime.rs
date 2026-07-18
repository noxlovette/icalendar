use crate::{
    params::{Fbtype, TimeZoneIdentifier, ValueDataType},
    properties::SharedParams,
    values::{DateOrDatetime, DateTime, Duration as DurationV, Period},
};

/// These params are shared by this module's component properties
struct DateTimeParams {
    shared: SharedParams,
    value_data_type: Option<ValueDataType>,
    tz_identifier: Option<TimeZoneIdentifier>,
}

/// This property defines the date and time that a to-do was actually
/// completed.
///
/// Example:
///
/// > COMPLETED:19960401T150000Z
///
/// [Section 3.8.2.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.1)
pub struct Completed {
    value: DateTime,
    params: SharedParams,
}

/// This property specifies the date and time that a calendar component ends.
///
/// Example:
///
/// > DTEND:19960401T150000Z
/// >
/// > DTEND;VALUE=DATE:19980704
///
/// [Section 3.8.2.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.2)
pub struct DateTimeEnd {
    value: DateOrDatetime,
    params: DateTimeParams,
}

/// This property defines the date and time that a to-do is expected to be
/// completed.
///
/// Example:
///
/// > DUE:19980430T000000Z
///
/// [Section 3.8.2.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.3)
pub struct DateTimeDue {
    value: DateOrDatetime,
    params: DateTimeParams,
}

/// This property specifies when the calendar component begins.
///
/// Example:
///
/// > DTSTART:19980118T073000Z
///
/// [Section 3.8.2.4](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.4)
pub struct DateTimeStart {
    value: DateOrDatetime,
    params: DateTimeParams,
}

/// This property specifies a positive duration of time.
///
/// Example:
///
/// > DURATION:PT1H0M0S
///
/// [Section 3.8.2.5](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.5)
pub struct Duration {
    value: DurationV,
    params: SharedParams,
}

/// This property defines one or more free or busy time intervals.
///
/// Example:
///
/// > FREEBUSY;FBTYPE=BUSY-UNAVAILABLE:19970308T160000Z/PT8H30M
///
/// [Section 3.8.2.6](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.6)
pub struct FreeBusyTime {
    value: Period,
    params: FreeBusyTimeParams,
}

struct FreeBusyTimeParams {
    shared: SharedParams,
    fb_time_type: Fbtype,
}

/// This property defines whether or not an event is transparent to busy time
/// searches.
///
/// Example:
///
/// > TRANSP:TRANSPARENT
///
/// [Section 3.8.2.7](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.2.7)
pub struct TimeTransparency {
    value: TranspValue,
    params: SharedParams,
}

/// Time transparency value for [`TimeTransparency`].
#[derive(Debug, Default)]
pub enum TranspValue {
    /// Event blocks busy-time searches. Default.
    #[default]
    Opaque,
    /// Event does not block busy-time searches.
    Transparent,
}
