//! RFC 5545 in Rust
#![warn(missing_docs)]

/// As specified [in the RFC Section 3.6](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6)
mod components;
/// Errors of the crate
mod error;
mod internal;
/// Sections 3.7 and 3.8 of the RFC
mod properties;
mod rrule;
pub use components::*;
pub use error::*;
pub use properties::*;
pub use rrule::*;

/// Alias for emails
pub type Email = String;

/// A globally unique identifier for a calendar component.
///
/// Typically generated from the current timestamp and a random suffix so it is
/// unique across calendar stores.  See [`Uid::new`] for the canonical
/// constructor.
pub struct Uid(pub String);

/// A pair of two values of the same type, used for properties such as [`Geo`] that carry two
/// coordinates.
pub struct Pair<T>(pub T, pub T);

/// A property can have attributes with which it is associated.  These
/// "property parameters" contain meta-information about the property or
/// the property value.  Property parameters are provided to specify such
/// information as the location of an alternate text representation for a
/// property value, the language of a text property value, the value type
/// of the property value, and other attributes.
///
/// Property parameter values that contain the COLON, SEMICOLON, or COMMA
/// character separators MUST be specified as quoted-string text values.
/// Property parameter values MUST NOT contain the DQUOTE character.  The
/// DQUOTE character is used as a delimiter for parameter values that
/// contain restricted characters or URI text.  For example:
///
/// > DESCRIPTION;ALTREP="cid:part1.0001@example.org":The Fall'98 Wild
/// > Wizards Conference - - Las Vegas\, NV\, USA
///
/// Property parameter values that are not in quoted-strings are
/// case-insensitive.
///
/// [More](https://datatracker.ietf.org/doc/html/rfc5545#autoid-11)
pub mod params;
/// The properties in an iCalendar object are strongly typed.  The
/// definition of each property restricts the value to be one of the
/// value data types, or simply value types, defined in this section.
/// The value type for a property will either be specified implicitly as
/// the default value type or will be explicitly specified with the
/// "VALUE" parameter.  If the value type of a property is one of the
/// alternate valid types, then it MUST be explicitly specified with the
/// "VALUE" parameter.
///
/// [More in the RFC](https://datatracker.ietf.org/doc/html/rfc5545#autoid-32)
pub mod values;

/// The Calendaring and Scheduling Core Object is a collection of
/// calendaring and scheduling information.  Typically, this information
/// will consist of an iCalendar stream with a single iCalendar object.
/// However, multiple iCalendar objects can be sequentially grouped
/// together in an iCalendar stream.  The first line and last line of the
/// iCalendar object MUST contain a pair of iCalendar object delimiter
/// strings.
///
/// The body of the iCalendar object consists of a sequence of calendar
/// properties and one or more calendar components.  The calendar
/// properties are attributes that apply to the calendar object as a
/// whole.  The calendar components are collections of properties that
/// express a particular calendar semantic.  For example, the calendar
/// component can specify an event, a to-do, a journal entry, time zone
/// information, free/busy time information, or an alarm.
///
/// An iCalendar object MUST include the "PRODID" and "VERSION" calendar
/// properties.  In addition, it MUST include at least one calendar
/// component.  Special forms of iCalendar objects are possible to
/// publish just busy time (i.e., only a "VFREEBUSY" calendar component)
/// or time zone (i.e., only a "VTIMEZONE" calendar component)
/// information.  In addition, a complex iCalendar object that is used to
/// capture a complete snapshot of the contents of a calendar is possible
/// (e.g., composite of many different calendar components).  More
/// commonly, an iCalendar object will consist of just a single "VEVENT",
/// "VTODO", or "VJOURNAL" calendar component.  Applications MUST ignore
/// x-comp and iana-comp values they don't recognize.  Applications that
/// support importing iCalendar objects SHOULD support all of the
/// component types defined in this document, and SHOULD NOT silently
/// drop any components as that can lead to user data loss.
pub struct ICalendar {
    prodid: String,
    version: String,
    calscale: Option<String>,
    method: Option<String>,
    component: Component,
}

impl Uid {
    /// Creates a new UID
    ///
    /// Taks the local time as rfc3339 and salts with nanoid
    pub fn new() -> Self {
        let now = chrono::Local::now();
        Self(format!(
            "{}-{}@ogonek.app",
            now.to_rfc3339(),
            nanoid::nanoid!(5),
        ))
    }

    /// Gets the UID as reference
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for Uid {
    fn default() -> Self {
        Self::new()
    }
}
