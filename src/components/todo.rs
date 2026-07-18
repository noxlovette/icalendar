use crate::properties::{
    Attachment, Attendee, Categories, Classification, Comment, Contact,
    DateTimeDue, DateTimeStamp, Description, Duration, ExceptionDateTimes, Geo,
    Iana, LastModified, Location, Organizer, PercentComplete, Priority, RRule,
    RecurrenceDateTimes, RecurrenceId, RelatedTo, RequestStatus, Resources,
    Sequence, Status, Summary, Uid, UniformResourceLocator, Xprop,
};

/// A "VTODO" calendar component is a grouping of component
/// properties and possibly "VALARM" calendar components that
/// represent an action-item or assignment.  For example, it can be
/// used to represent an item of work assigned to an individual; such
/// as "turn in travel expense today".
///
/// The "VTODO" calendar component cannot be nested within another
/// calendar component.  However, "VTODO" calendar components can be
/// related to each other or to a "VEVENT" or to a "VJOURNAL" calendar
/// component with the "RELATED-TO" property.
///
/// A "VTODO" calendar component without the "DTSTART" and "DUE" (or
/// "DURATION") properties specifies a to-do that will be associated
/// with each successive calendar date, until it is completed.
///
/// Example:  The following is an example of a "VTODO" calendar
/// component that needs to be completed before May 1st, 2007.  On
/// midnight May 1st, 2007 this to-do would be considered overdue.
///
/// > BEGIN:VTODO
/// >
/// > UID:20070313T123432Z-456553@example.com
/// >
/// > DTSTAMP:20070313T123432Z
/// >
/// > DUE;VALUE=DATE:20070501
/// >
/// > SUMMARY:Submit Quebec Income Tax Return for 2006
/// >
/// > CLASS:CONFIDENTIAL
/// >
/// > CATEGORIES:FAMILY,FINANCE
/// >
/// > STATUS:NEEDS-ACTION
/// >
/// > END:VTODO
/// >
///
/// [Section 3.6.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.2)
pub struct Todo {
    dtstamp: DateTimeStamp,
    uid: Uid,
    /// The following is REQUIRED if the component
    /// appears in an iCalendar object that doesn't
    /// specify the "METHOD" property; otherwise, it
    /// is OPTIONAL; in any case, it MUST NOT occur
    /// more than once.
    class: Option<Classification>,
    description: Option<Description>,
    geo: Option<Geo>,
    last_mod: Option<LastModified>,
    location: Option<Location>,
    organizer: Option<Organizer>,
    percent: Option<PercentComplete>,
    priority: Option<Priority>,
    recur_id: Option<RecurrenceId>,
    seq: Option<Sequence>,
    status: Option<Status>,
    summary: Option<Summary>,
    url: Option<UniformResourceLocator>,
    rrule: Option<RRule>,
    due: Option<DateTimeDue>,
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
