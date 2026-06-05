/// The calendar component carried by an [`crate::ICalendar`] object.
///
/// Each variant corresponds to a component type defined in RFC 5545 Section
/// 3.6.
pub enum Component {
    /// A scheduled event (`VEVENT`).
    VEvent(event::Eventc),
    /// A to-do task (`VTODO`).
    VTodo(todo::Todoc),
    /// A journal entry (`VJOURNAL`).
    VJournal(Journalc),
    /// Free/busy time information (`VFREEBUSY`).
    VFreeBusy(FreeBusyc),
    /// Time zone definition (`VTIMEZONE`).
    VTimezone(Timezonec),
}

mod event {
    use crate::{
        Email, Uid,
        descriptive::Classification,
        values::{DateTime, Duration, Recur, Uri},
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
    pub struct Eventc {
        dtstamp: DateTime,
        uid: Uid,
        /// The following is REQUIRED if the component
        /// appears in an iCalendar object that doesn't
        /// specify the "METHOD" property; otherwise, it
        /// is OPTIONAL; in any case, it MUST NOT occur
        /// more than once.
        dtstart: Option<DateTime>,
        class: Option<Classification>,
        description: Option<String>,
        geo: Option<String>,
        last_mod: Option<DateTime>,
        location: Option<String>,
        organizer: Option<Email>,
        priority: Option<i32>,
        seq: Option<i32>,
        status: Option<String>,
        summary: Option<String>,
        transp: Option<String>,
        url: Option<Uri>,
        recurid: Option<DateTime>,
        rrule: Recur,
        dtend: Option<DateTime>,
        duration: Option<Duration>,

        // Optional, can occur more than once
        attach: Vec<String>,
        attendee: Vec<Email>,
        categories: Vec<String>,
        comment: Vec<String>,
        contact: Vec<String>,
        exdate: Vec<DateTime>,
        rstatus: Vec<String>,
        related: Vec<String>,
        resources: Vec<String>,
        rdate: Vec<DateTime>,
    }
}

mod todo {
    use crate::{Email, Uid, values::DateTime};

    pub struct Todoc {
        dtstamp: DateTime,
        uid: Uid,
        class: Option<String>,
        completed: Option<DateTime>,
        created: Option<DateTime>,
        description: Option<String>,
        dtstart: Option<DateTime>,
        geo: Option<String>,
        last_mod: Option<DateTime>,
        location: Option<String>,
        organizer: Option<Email>,
        percent: Option<u8>,
        priority: Option<u8>,
        recurid: Option<DateTime>,
        seq: Option<u8>,
        status: Option<String>,
        summary: Option<String>,
    }
}
/// A `VJOURNAL` calendar component (stub).
pub struct Journalc;
/// A `VFREEBUSY` calendar component (stub).
pub struct FreeBusyc;
/// A `VTIMEZONE` calendar component (stub).
pub struct Timezonec;
