use crate::{
    Pair,
    params::{DataTypes, Encoding, Fmttype, Language, Params, TextParams},
    values::{Binary, Float, Integer, Text, Uri},
};

/// This property is used in "VEVENT", "VTODO", and "VJOURNAL" calendar
/// components to associate a resource (e.g., document) with the calendar
/// component.  This property is used in "VALARM" calendar components to
/// specify an audio sound resource or an email message attachment.  This
/// property can be specified as a URI pointing to a resource or as inline
/// binary encoded content.
///
/// When this property is specified as inline binary encoded content,
/// calendar applications MAY attempt to guess the media type of the resource
/// via inspection of its content if and only if the media type of the
/// resource is not given by the "FMTTYPE" parameter.  If the media type
/// remains unknown, calendar applications SHOULD treat it as type
/// "application/octet-stream".
///
/// [Section 3.8.1.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.1)
pub enum Attachment {
    /// Attachment referenced by a URI.
    Uri {
        /// The URI pointing to the resource.
        value: Uri,
        /// Optional format-type and encoding parameters.
        params: Params<AttachmentParams>,
    },
    /// Attachment with inline BASE64-encoded binary content.
    Binary {
        /// The inline binary data.
        value: Binary,
        /// Encoding and format-type parameters; `ENCODING=BASE64` is required.
        params: Params<AttachmentParams>,
    },
}

/// Parameter bundle for [`Attachment`].
#[derive(Default)]
pub struct AttachmentParams {
    fmttype: Option<Fmttype>,
    encoding: Option<Encoding>,
    value: Option<DataTypes>,
}

/// This property is used to specify categories or subtypes of the calendar
/// component.  The categories are useful in searching for a calendar
/// component of a particular type and category.  Within the "VEVENT",
/// "VTODO", or "VJOURNAL" calendar components, more than one category can
/// be specified as a COMMA-separated list of categories.
///
/// Example:
///
/// > CATEGORIES:APPOINTMENT,EDUCATION
///
/// [Section 3.8.1.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.2)
pub struct Categories {
    value: Vec<Text>,
    params: Params<Option<Language>>,
}

/// An access classification is only one component of the general security
/// system within a calendar application.  It provides a method of capturing
/// the scope of the access the calendar owner intends for information within
/// an individual calendar entry.  The access classification of an individual
/// iCalendar component is useful when measured along with the other security
/// components of a calendar system (e.g., calendar user authentication,
/// authorization, access rights, access role, etc.).
///
/// Hence, the semantics of the individual access classifications cannot be
/// completely defined by this memo alone.  Additionally, due to the "blind"
/// nature of most exchange processes using this memo, these access
/// classifications cannot serve as an enforcement statement for a system
/// receiving an iCalendar object.  Rather, they provide a method for
/// capturing the intention of the calendar owner for the access to the
/// calendar component.  If not specified in a component that allows this
/// property, the default value is PUBLIC.  Applications MUST treat x-name
/// and iana-token values they don't recognize the same way as they would the
/// PRIVATE value.
///
/// Example:
///
/// > CLASS:PUBLIC
///
/// [Section 3.8.1.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.3)
#[derive(Debug)]
pub enum Classification {
    /// Publicly accessible.
    Public {
        /// Property parameters.
        params: Params<()>,
    },
    /// Restricted to the calendar owner.
    Private {
        /// Property parameters.
        params: Params<()>,
    },
    /// Confidential; restricted access.
    Confidential {
        /// Property parameters.
        params: Params<()>,
    },
}

/// This property is used to specify a comment to the calendar user.
///
/// Example:
///
/// > COMMENT:The meeting really needs to include both the director and the vice-
/// >  president of the division.
///
/// [Section 3.8.1.4](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.4)
pub struct Comment {
    value: Text,
    params: Params<TextParams>,
}

/// This property is used in the "VEVENT" and "VTODO" to capture lengthy
/// textual descriptions associated with the activity.
///
/// This property is used in the "VJOURNAL" calendar component to capture one
/// or more textual journal entries.
///
/// This property is used in the "VALARM" calendar component to capture the
/// display text for a DISPLAY category of alarm, and to capture the body
/// text for an EMAIL category of alarm.
///
/// [Section 3.8.1.5](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.5)
pub struct Description {
    value: Text,
    params: Params<TextParams>,
}

/// This property value specifies latitude and longitude, in that order
/// (i.e., "LAT LON" ordering).  The longitude represents the location east
/// or west of the prime meridian as a positive or negative real number,
/// respectively.  The longitude and latitude values MAY be specified up to
/// six decimal places, which will allow for accuracy to within one meter of
/// geographical position.  Receiving applications MUST accept values of this
/// precision and MAY truncate values of greater precision.
///
/// Values for latitude and longitude shall be expressed as decimal fractions
/// of degrees.  Latitudes north of the equator and longitudes east of the
/// prime meridian are positive; south and west are negative.
///
/// Example:
///
/// > GEO:37.386013;-122.082932
///
/// [Section 3.8.1.6](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.6)
pub struct Geo {
    value: Pair<Float>,
    params: Params<()>,
}

/// Specific venues such as conference or meeting rooms may be explicitly
/// specified using this property.  An alternate representation may be
/// specified that is a URI that points to directory information with more
/// structured specification of the location.  For example, the alternate
/// representation may specify either an LDAP URL [RFC4516] pointing to an
/// LDAP server entry or a CID URL [RFC2392] pointing to a MIME body part
/// containing a Virtual-Information Card (vCard) [RFC2426] for the location.
///
/// Example:
///
/// > LOCATION:Conference Room - F123\, Bldg. 002
///
/// [Section 3.8.1.7](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.7)
pub struct Location {
    value: Text,
    params: TextParams,
}

/// The property value is a positive integer between 0 and 100.  A value of
/// "0" indicates the to-do has not yet been started.  A value of "100"
/// indicates that the to-do has been completed.  Integer values in between
/// indicate the percent partially complete.
///
/// When a to-do is assigned to multiple individuals, the property value
/// indicates the percent complete for that portion of the to-do assigned to
/// the assignee or delegatee.
///
/// Example:
///
/// > PERCENT-COMPLETE:39
///
/// [Section 3.8.1.8](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.8)
pub struct PercentComplete {
    value: Integer,
    params: Params<()>,
}

/// This priority is specified as an integer in the range 0 to 9.  A value
/// of 0 specifies an undefined priority.  A value of 1 is the highest
/// priority.  A value of 2 is the second highest priority.  Subsequent
/// numbers specify a decreasing ordinal priority.  A value of 9 is the
/// lowest priority.
///
/// A CUA with a three-level priority scheme of "HIGH", "MEDIUM", and "LOW"
/// is mapped into this property such that a property value in the range of
/// 1 to 4 specifies "HIGH" priority.  A value of 5 is the normal or
/// "MEDIUM" priority.  A value in the range of 6 to 9 is "LOW" priority.
///
/// Within a "VEVENT" calendar component, this property specifies a priority
/// for the event.  Within a "VTODO" calendar component, this property
/// specifies a priority for the to-do.
///
/// Example:
///
/// > PRIORITY:1
///
/// [Section 3.8.1.9](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.9)
pub struct Priority {
    value: Integer,
    params: Params<()>,
}

/// The property value is an arbitrary text.  More than one resource can be
/// specified as a COMMA-separated list of resources.
///
/// Example:
///
/// > RESOURCES:EASEL,PROJECTOR,VCR
///
/// [Section 3.8.1.10](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.10)
pub struct Resources {
    value: Text,
    params: TextParams,
}

/// In a group-scheduled calendar component, the property is used by the
/// "Organizer" to provide a confirmation of the event to the "Attendees".
/// For example in a "VEVENT" calendar component, the "Organizer" can
/// indicate that a meeting is tentative, confirmed, or cancelled.  In a
/// "VTODO" calendar component, the "Organizer" can indicate that an action
/// item needs action, is completed, is in process or being worked on, or has
/// been cancelled.  In a "VJOURNAL" calendar component, the "Organizer" can
/// indicate that a journal entry is draft, final, or has been cancelled or
/// removed.
///
/// Example:
///
/// > STATUS:TENTATIVE
///
/// [Section 3.8.1.11](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.11)
pub enum Status {
    /// Status of a `VEVENT` component.
    Event {
        /// The event status value.
        value: EventStatus,
        /// Property parameters.
        params: Params<()>,
    },
    /// Status of a `VTODO` component.
    Todo {
        /// The to-do status value.
        value: TodoStatus,
        /// Property parameters.
        params: Params<()>,
    },
    /// Status of a `VJOURNAL` component.
    Journal {
        /// The journal status value.
        value: JourStatus,
        /// Property parameters.
        params: Params<()>,
    },
}

/// Status values for a `VEVENT` component.
pub enum EventStatus {
    /// Event is tentatively scheduled.
    Tentative,
    /// Event is confirmed.
    Confirmed,
    /// Event has been cancelled.
    Cancelled,
}

/// Status values for a `VTODO` component.
pub enum TodoStatus {
    /// To-do has not yet been started.
    NeedsAction,
    /// To-do is complete.
    Completed,
    /// To-do is currently in progress.
    InProgress,
    /// To-do has been cancelled.
    Cancelled,
}

/// Status values for a `VJOURNAL` component.
pub enum JourStatus {
    /// Journal entry is a draft.
    Draft,
    /// Journal entry is final.
    Final,
    /// Journal entry has been cancelled.
    Cancelled,
}

/// This property is used in the "VEVENT", "VTODO", and "VJOURNAL" calendar
/// components to capture a short, one-line summary about the activity or
/// journal entry.
///
/// This property is used in the "VALARM" calendar component to capture the
/// subject of an EMAIL category of alarm.
///
/// Example:
///
/// > SUMMARY:Department Party
///
/// [Section 3.8.1.12](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.1.12)
pub struct Summary {
    value: Text,
    params: TextParams,
}
