use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::params::{Range, Recur};

/// Convenience wrapper for recurrence rules
pub struct RecurrenceSet {
    /// Purpose: This property is used in conjunction with the "UID" and
    /// "SEQUENCE" properties to identify a specific instance of a
    /// recurring "VEVENT", "VTODO", or "VJOURNAL" calendar component.
    /// The property value is the original value of the "DTSTART" property
    /// of the recurrence instance.
    ///
    /// Value Type:  The default value type is DATE-TIME.  The value type can
    /// set to a DATE value type.  This property MUST have the same
    /// type as the "DTSTART" property contained within the
    /// component.  Furthermore, this property MUST be specified
    /// a date with local time if and only if the "DTSTART" property
    /// within the recurring component is specified as a date
    /// local time.
    ///
    /// [More](https://datatracker.ietf.org/doc/html/rfc5545#autoid-93)
    pub recurrence_id: Option<DateTime<Utc>>,
    pub sequence: i32,
    /// The "RANGE" parameter is used to specify the effective range of
    /// recurrence instances from the instance specified by the
    /// "RECURRENCE-ID" property value.  The value for the range parameter
    /// can only be "THISANDFUTURE" to indicate a range defined by the
    /// given recurrence instance and all subsequent instances.
    /// Subsequent instances are determined by their "RECURRENCE-ID" value
    /// and not their current scheduled start time.  Subsequent instances
    /// defined in separate components are not impacted by the given
    /// recurrence instance.  When the given recurrence instance is
    /// rescheduled, all subsequent instances are also rescheduled by the
    /// same time difference.  For instance, if the given recurrence
    /// instance is rescheduled to start 2 hours later, then all
    /// subsequent instances are also rescheduled 2 hours later.
    pub range: Range,

    /// This value type is used to identify properties that contain
    /// a recurrence rule specification
    pub recur: Recur,
}

/// RRULE parse/validation errors.
#[derive(Error, Debug)]
pub enum RRuleError {
    /// RRULE syntax is invalid.
    #[error("Некорректный формат RRULE: {0}")]
    InvalidFormat(String),
    /// Required RRULE part is missing.
    #[error("Отсутствует обязательное поле: {0}")]
    MissingField(String),
    /// Frequency value is unsupported.
    #[error("Неподдерживаемая частота: {0}")]
    UnsupportedFrequency(String),
    /// Interval value is invalid.
    #[error("Некорректный интервал: {0}")]
    InvalidInterval(String),
    /// BYDAY value contains unknown day token.
    #[error("Некорректный день: {0}")]
    InvalidDay(String),
    /// COUNT value is invalid.
    #[error("Некорректное количество: {0}")]
    InvalidCount(String),
    /// UNTIL value has unsupported date format.
    #[error("Некорректная дата until: {0}")]
    InvalidUntilDate(String),
}
