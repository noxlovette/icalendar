mod event;
mod free_busy;
mod journal;
mod timezone;
mod todo;

pub use event::Event;
pub use free_busy::FreeBusy;
pub use timezone::Timezone;

/// The calendar component carried by an [`crate::ICalendar`] object.
///
/// Each variant corresponds to a component type defined in RFC 5545 Section
/// 3.6.
pub enum Component {
    /// A scheduled event (`VEVENT`).
    Event,
    /// A to-do task (`VTODO`).
    Todo,
    /// A journal entry (`VJOURNAL`).
    Journal,
    /// Free/busy time information (`VFREEBUSY`).
    FreeBusy,
    /// Time zone definition (`VTIMEZONE`).
    Timezone,
}
