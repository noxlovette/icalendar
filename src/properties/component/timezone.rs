use crate::{
    params::{Language, Params},
    values::{Text, Uri, UtcOffset},
};

/// This property specifies the text value that uniquely identifies the
/// "VTIMEZONE" calendar component in the scope of an iCalendar object.
///
/// Example:
///
/// > TZID:America/New_York
///
/// [Section 3.8.3.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.1)
pub struct TimeZoneIdentifier {
    value: Text,
    params: Params<()>,
}

/// This property specifies the customary designation for a time zone
/// description.
///
/// Example:
///
/// > TZNAME:EST
///
/// [Section 3.8.3.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.2)
pub struct TimeZoneName {
    value: Text,
    params: Params<Option<Language>>,
}

/// This property specifies the offset that is in use prior to this time zone
/// observance.
///
/// Example:
///
/// > TZOFFSETFROM:-0500
///
/// [Section 3.8.3.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.3)
pub struct TimeZoneOffsetFrom {
    value: UtcOffset,
    params: Params<()>,
}

/// This property specifies the UTC offset that is in use in this time zone
/// observance.
///
/// Example:
///
/// > TZOFFSETTO:-0400
///
/// [Section 3.8.3.4](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.4)
pub struct TimeZoneOffsetTo {
    value: UtcOffset,
    params: Params<()>,
}

/// This property provides a means for a VTIMEZONE component to point to a
/// network location that can be used to retrieve an up-to-date version of
/// itself.
///
/// Example:
///
/// > TZURL:http://timezones.example.org/tz/America-Los_Angeles.ics
///
/// [Section 3.8.3.5](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.3.5)
pub struct TimeZoneUrl {
    value: Uri,
    params: Params<()>,
}
