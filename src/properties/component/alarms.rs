use crate::{
    ast::parser::ParseError,
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
#[derive(Debug)]
pub struct Action {
    value: ActionEnum,
    params: SharedParams,
}

/// Possible alarm actions for [`Action`].
#[derive(Debug)]
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

impl TryFrom<&[u8]> for ActionEnum {
    type Error = ParseError;
    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        let r = match v {
            b"AUDIO" => Self::Audio,
            b"DISPLAY" => Self::Display,
            b"EMAIL" => Self::Email,
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
}

/// This property defines the number of times the alarm should be repeated,
/// after the initial trigger.
///
/// Example:
///
/// > REPEAT:4
///
/// [Section 3.8.6.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.6.2)
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Trigger {
    value: DateTimeDuration,
    params: TriggerParams,
}

#[derive(Debug)]
struct TriggerParams {
    shared: SharedParams,
    value_data_type: Option<ValueDataType>,
    tz_identifier: Option<TimeZoneIdentifier>,
    trigger_relationship: Option<AlarmTriggerRelationship>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_fixed_tokens() {
        assert!(matches!(
            ActionEnum::try_from(b"AUDIO".as_slice()),
            Ok(ActionEnum::Audio)
        ));
        assert!(matches!(
            ActionEnum::try_from(b"DISPLAY".as_slice()),
            Ok(ActionEnum::Display)
        ));
        assert!(matches!(
            ActionEnum::try_from(b"EMAIL".as_slice()),
            Ok(ActionEnum::Email)
        ));
    }

    #[test]
    fn action_x_name_and_iana() {
        assert!(matches!(
            ActionEnum::try_from(b"X-CUSTOM".as_slice()),
            Ok(ActionEnum::XName(_))
        ));
        assert!(matches!(
            ActionEnum::try_from(b"PROCEDURE".as_slice()),
            Ok(ActionEnum::Iana(_))
        ));
    }
}
