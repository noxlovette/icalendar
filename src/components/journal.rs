use crate::properties::{
    Attachment, Attendee, Categories, Classification, Comment, Contact,
    DateTimeCreated, DateTimeStamp, DateTimeStart, Description,
    ExceptionDateTimes, Iana, LastModified, Organizer, RRule,
    RecurrenceDateTimes, RecurrenceId, RelatedTo, RequestStatus, Sequence,
    Status, Summary, Uid, UniformResourceLocator, Xprop,
};

/// A "VJOURNAL" calendar component is a grouping of
/// component properties that represent one or more descriptive text
/// notes associated with a particular calendar date.  The "DTSTART"
/// property is used to specify the calendar date with which the
/// journal entry is associated.  Generally, it will have a DATE value
/// data type, but it can also be used to specify a DATE-TIME value
/// data type.  Examples of a journal entry include a daily record of
/// a legislative body or a journal entry of individual telephone
/// contacts for the day or an ordered list of accomplishments for the
/// day.  The "VJOURNAL" calendar component can also be used to
/// associate a document with a calendar date.
///
/// The "VJOURNAL" calendar component does not take up time on a
/// calendar.  Hence, it does not play a role in free or busy time
/// searches -- it is as though it has a time transparency value of
/// TRANSPARENT.  It is transparent to any such searches.
///
/// The "VJOURNAL" calendar component cannot be nested within another
/// calendar component.  However, "VJOURNAL" calendar components can
/// be related to each other or to a "VEVENT" or to a "VTODO" calendar
/// component, with the "RELATED-TO" property.
///
/// Example:  The following is an example of the "VJOURNAL" calendar
/// component:
///
/// > BEGIN:VJOURNAL
/// >
/// > UID:19970901T130000Z-123405@example.com
/// >
/// > DTSTAMP:19970901T130000Z
/// >
/// > DTSTART;VALUE=DATE:19970317
/// >
/// > SUMMARY:Staff meeting minutes
/// >
/// > DESCRIPTION:1. Staff meeting: Participants include Joe\,
/// > Lisa\, and Bob. Aurora project plans were reviewed.
/// > There is currently no budget reserves for this project.
/// > Lisa will escalate to management. Next meeting on Tuesday.\n
/// > 2. Telephone Conference: ABC Corp. sales representative
/// > called to discuss new printer. Promised to get us a demo by
/// > Friday.\n3. Henry Miller (Handsoff Insurance): Car was
/// > totaled by tree. Is looking into a loaner car. 555-2323
/// > (tel).
/// >
/// > END:VJOURNAL
/// >
///
/// [Section 3.6.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.6.3)
pub struct VJournal {
    dtstamp: DateTimeStamp,
    uid: Uid,
    class: Option<Classification>,
    created: Option<DateTimeCreated>,
    dtstart: Option<DateTimeStart>,
    last_mod: Option<LastModified>,
    organizer: Option<Organizer>,
    recurid: Option<RecurrenceId>,
    seq: Option<Sequence>,
    status: Option<Status>,
    summary: Option<Summary>,
    url: Option<UniformResourceLocator>,
    rrule: Option<RRule>,
    attach: Vec<Attachment>,
    attendee: Vec<Attendee>,
    categories: Vec<Categories>,
    comment: Vec<Comment>,
    contact: Vec<Contact>,
    description: Vec<Description>,
    exdate: Vec<ExceptionDateTimes>,
    related: Vec<RelatedTo>,
    rdate: Vec<RecurrenceDateTimes>,
    rstatus: Vec<RequestStatus>,
    xprop: Xprop,
    iana: Iana,
}
