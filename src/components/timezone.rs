use crate::properties::{
    Comment, DateTimeStart, Iana, LastModified, RRule, RecurrenceDateTimes,
    TimeZoneIdentifier, TimeZoneName, TimeZoneOffsetFrom, TimeZoneOffsetTo,
    TimeZoneUrl, Xprop,
};

/// A time zone is unambiguously defined by the set of time
/// measurement rules determined by the governing body for a given
/// geographic area.  These rules describe, at a minimum, the base
/// offset from UTC for the time zone, often referred to as the
/// Standard Time offset.  Many locations adjust their Standard Time
/// forward or backward by one hour, in order to accommodate seasonal
/// changes in number of daylight hours, often referred to as Daylight
/// Saving Time.  Some locations adjust their time by a fraction of an
/// hour.  Standard Time is also known as Winter Time.  Daylight
/// Saving Time is also known as Advanced Time, Summer Time, or Legal
/// Time in certain countries.
///
/// Interoperability between two calendaring and scheduling
/// applications, especially for recurring events, to-dos or journal
/// entries, is dependent on the ability to capture and convey date
/// and time information in an unambiguous format.  The specification
/// of current time zone information is integral to this behavior.
///
/// If present, the "VTIMEZONE" calendar component defines the set of
/// Standard Time and Daylight Saving Time observances (or rules) for
/// a particular time zone for a given interval of time.  The
/// "VTIMEZONE" calendar component cannot be nested within other
/// calendar components.  Multiple "VTIMEZONE" calendar components can
/// exist in an iCalendar object.  In this situation, each "VTIMEZONE"
/// MUST represent a unique time zone definition.  This is necessary
/// for some classes of events, such as airline flights, that start in
/// one time zone and end in another.
///
/// The "VTIMEZONE" calendar component MUST include the "TZID"
/// property and at least one definition of a "STANDARD" or "DAYLIGHT"
/// sub-component.  The "STANDARD" or "DAYLIGHT" sub-component MUST
/// include the "DTSTART", "TZOFFSETFROM", and "TZOFFSETTO"
/// properties.
///
/// An individual "VTIMEZONE" calendar component MUST be specified for
/// each unique "TZID" parameter value specified in the iCalendar
/// object.  In addition, a "VTIMEZONE" calendar component, referred
/// to by a recurring calendar component, MUST provide valid time zone
/// information for all recurrence instances.
///
/// Example:  This is a simple example showing the current time zone
/// rules for New York City using only the "DTSTART" property, suitable
/// for a recurring event that starts on or later than March 11, 2007
/// at 03:00:00 EDT and ends no later than March 9, 2008 at 01:59:59
/// EST.
///
/// > BEGIN:VTIMEZONE
/// >
/// > TZID:America/New_York
/// >
/// > LAST-MODIFIED:20050809T050000Z
/// >
/// > BEGIN:STANDARD
/// >
/// > DTSTART:20071104T020000
/// >
/// > TZOFFSETFROM:-0400
/// >
/// > TZOFFSETTO:-0500
/// >
/// > TZNAME:EST
/// >
/// > END:STANDARD
/// >
/// > BEGIN:DAYLIGHT
/// >
/// > DTSTART:20070311T020000
/// >
/// > TZOFFSETFROM:-0500
/// >
/// > TZOFFSETTO:-0400
/// >
/// > TZNAME:EDT
/// >
/// > END:DAYLIGHT
/// >
/// > END:VTIMEZONE
/// >
///
/// [Section 3.6.5](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.5)
pub struct Timezone {
    tzid: TimeZoneIdentifier,
    last_mod: Option<LastModified>,
    tz_url: Option<TimeZoneUrl>,
    standatdc: TzProp,
    daylightc: TzProp,
    xprop: Xprop,
    iana: Iana,
}

struct TzProp {
    dtstart: DateTimeStart,
    tz_offset_to: TimeZoneOffsetTo,
    tz_offset_from: TimeZoneOffsetFrom,
    rrule: Option<RRule>,
    comment: Comment,
    rdate: RecurrenceDateTimes,
    tzname: TimeZoneName,
    xprop: Xprop,
    iana: Iana,
}
