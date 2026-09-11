use crate::{
    components::alarm::Alarm,
    properties::{
        Attachment, Attendee, Categories, Classification, Comment, Completed,
        Contact, DateTimeCreated, DateTimeDue, DateTimeStamp, DateTimeStart,
        Description, Duration, ExceptionDateTimes, Geo, Iana, LastModified,
        Location, Organizer, PercentComplete, Priority, RRule,
        RecurrenceDateTimes, RecurrenceId, RelatedTo, RequestStatus, Resources,
        Sequence, Status, Summary, Uid, UniformResourceLocator, Xprop,
    },
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
#[derive(Debug)]
pub struct Todo {
    pub(crate) dtstamp: DateTimeStamp,
    pub(crate) uid: Uid,
    pub(crate) class: Option<Classification>,
    pub(crate) completed: Option<Completed>,
    pub(crate) created: Option<DateTimeCreated>,
    pub(crate) description: Option<Description>,
    pub(crate) dtstart: Option<DateTimeStart>,
    pub(crate) geo: Option<Geo>,
    pub(crate) last_mod: Option<LastModified>,
    pub(crate) location: Option<Location>,
    pub(crate) organizer: Option<Organizer>,
    pub(crate) percent: Option<PercentComplete>,
    pub(crate) priority: Option<Priority>,
    pub(crate) recur_id: Option<RecurrenceId>,
    pub(crate) seq: Option<Sequence>,
    pub(crate) status: Option<Status>,
    pub(crate) summary: Option<Summary>,
    pub(crate) url: Option<UniformResourceLocator>,
    pub(crate) rrule: Option<RRule>,
    pub(crate) due: Option<DateTimeDue>,
    pub(crate) duration: Option<Duration>,
    pub(crate) attach: Vec<Attachment>,
    pub(crate) attendee: Vec<Attendee>,
    pub(crate) categories: Vec<Categories>,
    pub(crate) comment: Vec<Comment>,
    pub(crate) contact: Vec<Contact>,
    pub(crate) exdate: Vec<ExceptionDateTimes>,
    pub(crate) rstatus: Vec<RequestStatus>,
    pub(crate) related: Vec<RelatedTo>,
    pub(crate) resources: Vec<Resources>,
    pub(crate) rdate: Vec<RecurrenceDateTimes>,
    pub(crate) xprop: Vec<Xprop>,
    pub(crate) iana: Vec<Iana>,
    pub(crate) alarms: Vec<Alarm>,
}
