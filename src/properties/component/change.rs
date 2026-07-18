use crate::{
    Params,
    values::{DateTime, Integer},
};

/// This property specifies the date and time that the calendar information
/// was created by the calendar user agent in the calendar store.
///
/// Example:
///
/// > CREATED:19960329T133000Z
///
/// [Section 3.8.7.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.1)
pub struct DateTimeCreated {
    value: DateTime,
    params: Params,
}

/// This property specifies the date and time that the instance of the iCalendar
/// object was created (when `METHOD` is present), or the date and time that the
/// calendar component was last revised in the calendar store (when `METHOD` is
/// absent).
///
/// Example:
///
/// > DTSTAMP:19971210T080000Z
///
/// [Section 3.8.7.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.2)
pub struct DateTimeStamp {
    value: DateTime,
    params: Params,
}

/// This property specifies the date and time that the information associated
/// with the calendar component was last revised in the calendar store.
///
/// Example:
///
/// > LAST-MODIFIED:19960817T133000Z
///
/// [Section 3.8.7.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.3)
pub struct LastModified {
    value: DateTime,
    params: Params,
}

/// This property defines the revision sequence number of the calendar component
/// within a sequence of revisions.
///
/// Example:
///
/// > SEQUENCE:0
///
/// [Section 3.8.7.4](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.7.4)
pub struct Sequence {
    value: Integer,
    params: Params,
}
