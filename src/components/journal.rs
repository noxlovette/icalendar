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
#[derive(Debug)]
pub struct Journal {
    pub(crate) dtstamp: DateTimeStamp,
    pub(crate) uid: Uid,
    pub(crate) class: Option<Classification>,
    pub(crate) created: Option<DateTimeCreated>,
    pub(crate) dtstart: Option<DateTimeStart>,
    pub(crate) last_mod: Option<LastModified>,
    pub(crate) organizer: Option<Organizer>,
    pub(crate) recurid: Option<RecurrenceId>,
    pub(crate) seq: Option<Sequence>,
    pub(crate) status: Option<Status>,
    pub(crate) summary: Option<Summary>,
    pub(crate) url: Option<UniformResourceLocator>,
    pub(crate) rrule: Option<RRule>,
    pub(crate) attach: Vec<Attachment>,
    pub(crate) attendee: Vec<Attendee>,
    pub(crate) categories: Vec<Categories>,
    pub(crate) comment: Vec<Comment>,
    pub(crate) contact: Vec<Contact>,
    pub(crate) description: Vec<Description>,
    pub(crate) exdate: Vec<ExceptionDateTimes>,
    pub(crate) related: Vec<RelatedTo>,
    pub(crate) rdate: Vec<RecurrenceDateTimes>,
    pub(crate) rstatus: Vec<RequestStatus>,
    pub(crate) xprop: Vec<Xprop>,
    pub(crate) iana: Vec<Iana>,
}
