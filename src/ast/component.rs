/// The calendar component carried by an [`crate::ICalendar`] object.
///
/// Each variant corresponds to a component type defined in RFC 5545 Section
/// 3.6.
pub enum Component {
    /// A scheduled event (`VEVENT`).
    VEvent,
    /// A to-do task (`VTODO`).
    VTodo,
    /// A journal entry (`VJOURNAL`).
    VJournal,
    /// Free/busy time information (`VFREEBUSY`).
    VFreeBusy,
    /// Time zone definition (`VTIMEZONE`).
    VTimezone,
}
