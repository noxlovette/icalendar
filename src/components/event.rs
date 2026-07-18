use crate::properties::{
    Attachment, Attendee, Categories, Classification, Comment, Contact,
    DateTimeEnd, DateTimeStamp, DateTimeStart, Description, Duration,
    ExceptionDateTimes, Geo, Iana, LastModified, Location, Organizer, Priority,
    RRule, RecurrenceDateTimes, RecurrenceId, RelatedTo, RequestStatus,
    Resources, Sequence, Status, Summary, Uid, UniformResourceLocator, Xprop,
};

/// A "VEVENT" calendar component is a grouping of
/// component properties, possibly including "VALARM" calendar
/// components, that represents a scheduled amount of time on a
/// calendar.  For example, it can be an activity; such as a one-hour
/// long, department meeting from 8:00 AM to 9:00 AM, tomorrow.
/// Generally, an event will take up time on an individual calendar.
/// Hence, the event will appear as an opaque interval in a search for
/// busy time.  Alternately, the event can have its Time Transparency
/// set to "TRANSPARENT" in order to prevent blocking of the event in
/// searches for busy time.
///
/// The "VEVENT" is also the calendar component used to specify an
/// anniversary or daily reminder within a calendar.  These events
/// have a DATE value type for the "DTSTART" property instead of the
/// default value type of DATE-TIME.  If such a "VEVENT" has a "DTEND"
/// property, it MUST be specified as a DATE value also.  The
/// anniversary type of "VEVENT" can span more than one date (i.e.,
/// "DTEND" property value is set to a calendar date after the
/// "DTSTART" property value).  If such a "VEVENT" has a "DURATION"
/// property, it MUST be specified as a "dur-day" or "dur-week" value.
///
/// The "DTSTART" property for a "VEVENT" specifies the inclusive
/// start of the event.  For recurring events, it also specifies the
/// very first instance in the recurrence set.  The "DTEND" property
/// for a "VEVENT" calendar component specifies the non-inclusive end
/// of the event.  For cases where a "VEVENT" calendar component
/// specifies a "DTSTART" property with a DATE value type but no
/// "DTEND" nor "DURATION" property, the event's duration is taken to
/// be one day.  For cases where a "VEVENT" calendar component
/// specifies a "DTSTART" property with a DATE-TIME value type but no
/// "DTEND" property, the event ends on the same calendar date and
/// time of day specified by the "DTSTART" property.
///
/// The "VEVENT" calendar component cannot be nested within another
/// calendar component.  However, "VEVENT" calendar components can be
/// related to each other or to a "VTODO" or to a "VJOURNAL" calendar
/// component with the "RELATED-TO" property.
///
///
/// Example:  The following is an example of the "VEVENT" calendar
/// component used to represent a meeting that will also be opaque to
/// searches for busy time:
///
/// > BEGIN:VEVENT
/// >
/// > UID:19970901T130000Z-123401@example.com
/// >
/// > DTSTAMP:19970901T130000Z
/// >
/// > DTSTART:19970903T163000Z
/// >
/// > DTEND:19970903T190000Z
/// >
/// > SUMMARY:Annual Employee Review
/// >
/// > CLASS:PRIVATE
/// >
/// > CATEGORIES:BUSINESS,HUMAN RESOURCES
/// >
/// > END:VEVENT
/// >
///
/// The following is an example of the "VEVENT" calendar component
/// used to represent a reminder that will not be opaque, but rather
/// transparent, to searches for busy time:
///
/// > BEGIN:VEVENT
/// >
/// > UID:19970901T130000Z-123402@example.com
/// >
/// > DTSTAMP:19970901T130000Z
/// >
/// > DTSTART:19970401T163000Z
/// >
/// > DTEND:19970402T010000Z
/// >
/// > SUMMARY:Laurel is in sensitivity awareness class.
/// >
/// > CLASS:PUBLIC
/// >
/// > CATEGORIES:BUSINESS,HUMAN RESOURCES
/// >
/// > TRANSP:TRANSPARENT
/// >
/// > END:VEVENT
/// >
///
/// The following is an example of the "VEVENT" calendar component
/// used to represent an anniversary that will occur annually:
///
/// > BEGIN:VEVENT
/// >
/// > UID:19970901T130000Z-123403@example.com
/// >
/// > DTSTAMP:19970901T130000Z
/// >
/// > DTSTART;VALUE=DATE:19971102
/// >
/// > SUMMARY:Our Blissful Anniversary
/// >
/// > TRANSP:TRANSPARENT
/// >
/// > CLASS:CONFIDENTIAL
/// >
/// > CATEGORIES:ANNIVERSARY,PERSONAL,SPECIAL OCCASION
/// >
/// > RRULE:FREQ=YEARLY
/// >
/// > END:VEVENT
/// >
/// The following is an example of the "VEVENT" calendar component
/// used to represent a multi-day event scheduled from June 28th, 2007
/// to July 8th, 2007 inclusively.  Note that the "DTEND" property is
/// set to July 9th, 2007, since the "DTEND" property specifies the
/// non-inclusive end of the event.
///
/// > BEGIN:VEVENT
/// >
/// > UID:20070423T123432Z-541111@example.com
/// >
/// > DTSTAMP:20070423T123432Z
/// >
/// > DTSTART;VALUE=DATE:20070628
/// >
/// > DTEND;VALUE=DATE:20070709
/// >
/// > SUMMARY:Festival International de Jazz de Montreal
/// >
/// > TRANSP:TRANSPARENT
/// >
/// > END:VEVENT
/// >
///
/// [Section 3.6.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.1)
pub struct Event {
    dtstamp: DateTimeStamp,
    uid: Uid,
    /// The following is REQUIRED if the component
    /// appears in an iCalendar object that doesn't
    /// specify the "METHOD" property; otherwise, it
    /// is OPTIONAL; in any case, it MUST NOT occur
    /// more than once.
    dtstart: Option<DateTimeStart>,
    class: Option<Classification>,
    description: Option<Description>,
    geo: Option<Geo>,
    last_mod: Option<LastModified>,
    location: Option<Location>,
    organizer: Option<Organizer>,
    priority: Option<Priority>,
    seq: Option<Sequence>,
    status: Option<Status>,
    summary: Option<Summary>,
    transp: Option<String>,
    url: Option<UniformResourceLocator>,
    recurid: Option<RecurrenceId>,
    rrule: Option<RRule>,
    dtend: Option<DateTimeEnd>,
    duration: Option<Duration>,
    attach: Vec<Attachment>,
    attendee: Vec<Attendee>,
    categories: Vec<Categories>,
    comment: Vec<Comment>,
    contact: Vec<Contact>,
    exdate: Vec<ExceptionDateTimes>,
    rstatus: Vec<RequestStatus>,
    related: Vec<RelatedTo>,
    resources: Vec<Resources>,
    rdate: Vec<RecurrenceDateTimes>,
    xprop: Xprop,
    iana: Iana,
}
