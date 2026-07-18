use crate::{
    params::{AlarmTriggerRelationship, TimeZoneIdentifier, ValueDataType},
    properties::SharedParams,
    values::{DateTimeDuration, Integer, Text},
};

/// This property defines the action to be invoked when an alarm is triggered.
///
/// Example:
///
/// > ACTION:AUDIO
///
/// [Section 3.8.6.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.1)
pub struct Action {
    value: ActionEnum,
    params: SharedParams,
}

/// Possible alarm actions for [`Action`].
pub enum ActionEnum {
    /// Play an audio clip.
    Audio,
    /// Display a text message.
    Display,
    /// Send an email message.
    Email,
    /// An IANA-registered action.
    Iana(Text),
    /// A non-standard `X-` prefixed action.
    XName(Text),
}

/// This property defines the number of times the alarm should be repeated,
/// after the initial trigger.
///
/// Example:
///
/// > REPEAT:4
///
/// [Section 3.8.6.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.2)
pub struct Repeat {
    value: Integer,
    params: SharedParams,
}

/// This property specifies when an alarm will trigger.
///
/// Example:
///
/// > TRIGGER:-PT15M
/// >
/// > TRIGGER;RELATED=END:PT5M
///
/// [Section 3.8.6.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.3)
pub struct Trigger {
    value: DateTimeDuration,
    params: TriggerParams,
}

struct TriggerParams {
    shared: SharedParams,
    value_data_type: Option<ValueDataType>,
    tz_identifier: Option<TimeZoneIdentifier>,
    trigger_relationship: Option<AlarmTriggerRelationship>,
}
