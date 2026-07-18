use crate::{
    params::Params,
    values::{DateOrDatetime, DateTimePeriod, Recur},
};
use params::*;

/// This property defines the list of DATE-TIME exceptions for recurring events,
/// to-dos, journal entries, or time zone definitions.
///
/// Example:
///
/// > EXDATE:19960402T010000Z,19960403T010000Z,19960404T010000Z
///
/// [Section 3.8.5.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.1)
pub struct ExceptionDateTimes {
    value: Vec<DateOrDatetime>,
    params: Params,
}

/// This property defines the list of DATE-TIME values for recurring events,
/// to-dos, journal entries, or time zone definitions.
///
/// Example:
///
/// > RDATE;TZID=America/New_York:19970714T083000
///
/// [Section 3.8.5.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.2)
pub struct RecurrenceDateTimes {
    value: Vec<DateTimePeriod>,
    params: Params,
}

/// This property defines a rule or repeating pattern for recurring events,
/// to-dos, journal entries, or time zone definitions.
///
/// Example:
///
/// > RRULE:FREQ=DAILY;COUNT=10
///
/// [Section 3.8.5.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5.3)
pub struct RRule {
    value: Recur,
    params: Params,
}

mod params {
    use crate::params::{DataTypes, TimeZoneIdentifier};

    /// Parameter bundle for [`ExceptionDateTimes`].
    #[derive(Default)]
    pub struct ExDateParams {
        data_type: Option<DataTypes>,
        tzid: Option<TimeZoneIdentifier>,
    }

    /// Parameter bundle for [`RecurrenceDateTimes`].
    #[derive(Default)]
    pub struct RDateParams {
        data_type: Option<DataTypes>,
        tzid: Option<TimeZoneIdentifier>,
    }
}
