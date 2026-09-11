use crate::{
    components::{
        event::Event, free_busy::FreeBusy, journal::Journal,
        timezone::Timezone, todo::Todo,
    },
    properties::{
        CalendarScale, Iana, Method, ProductIdentifier, Version, Xprop,
    },
};

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
#[derive(Debug)]
pub struct Calendar {
    pub(crate) prodid: ProductIdentifier,
    pub(crate) version: Version,
    pub(crate) calscale: Option<CalendarScale>,
    pub(crate) method: Option<Method>,
    pub(crate) xprop: Vec<Xprop>,
    pub(crate) iana: Vec<Iana>,
    pub(crate) components: Vec<Component>,
}

/// The calendar component carried by a built [`Calendar`] — the built
/// counterpart of [`crate::ast::Component`], which wraps the in-progress
/// builders instead. One variant per component type RFC 5545 §3.6 allows
/// directly under `VCALENDAR`; see [`crate::ast`]'s module docs for why a
/// sub-component (`VALARM`, `STANDARD`, `DAYLIGHT`) never gets a variant
/// here.
#[derive(Debug)]
pub enum Component {
    /// A scheduled event (`VEVENT`).
    Event(Event),
    /// A to-do task (`VTODO`).
    Todo(Todo),
    /// A journal entry (`VJOURNAL`).
    Journal(Journal),
    /// Free/busy time information (`VFREEBUSY`).
    FreeBusy(FreeBusy),
    /// Time zone definition (`VTIMEZONE`).
    Timezone(Timezone),
}

impl Calendar {
    /// The calendar components (`VEVENT`, `VTODO`, `VJOURNAL`, `VFREEBUSY`,
    /// `VTIMEZONE`) carried by this `VCALENDAR` object.
    pub fn components(&self) -> &[Component] {
        &self.components
    }
}
