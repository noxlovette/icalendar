use crate::{
    params::{CommonName, DirectoryEntryReference, Language, Params, SentBy},
    values::{CalendarUserAddress, DateOrDatetime, Text, Uri},
};
use params::*;

/// This property defines an "Attendee" within a calendar component.
///
/// Example:
///
/// > ATTENDEE;ROLE=REQ-PARTICIPANT;DELEGATED-FROM="mailto:bob@example.com";
/// > PARTSTAT=ACCEPTED;CN=Jane Doe:mailto:jdoe@example.com
///
/// [Section 3.8.4.1](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.1)
pub struct Attendee {
    value: CalendarUserAddress,
    params: Params,
}

/// This property is used to represent contact information or alternately a
/// reference to contact information associated with the calendar component.
///
/// Example:
///
/// > CONTACT:Jim Dolittle\, ABC Industries\, +1-919-555-1234
///
/// [Section 3.8.4.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.2)
pub struct Contact {
    value: Text,
    params: Params,
}

/// This property defines the organizer for a calendar component.
///
/// Example:
///
/// > ORGANIZER;CN=John Smith:mailto:jsmith@example.com
///
/// [Section 3.8.4.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.3)
pub struct Organizer {
    value: CalendarUserAddress,
    params: Params,
}

/// Parameter bundle for [`Organizer`].
#[derive(Default)]
pub struct OrgParams {
    language: Option<Language>,
    common_name: Option<CommonName>,
    directory: Option<DirectoryEntryReference>,
    sent_by: Option<SentBy>,
}

/// This property is used in conjunction with the "UID" and "SEQUENCE"
/// property to identify a particular instance of a recurring event, to-do,
/// or journal.
///
/// Example:
///
/// > RECURRENCE-ID;VALUE=DATE:19960401
///
/// [Section 3.8.4.4](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.4)
pub struct RecurrenceId {
    value: DateOrDatetime,
    params: Params,
}

/// This property is used to represent a relationship or reference between
/// one calendar component and another.  The property value consists of the
/// persistent, globally unique identifier of another calendar component.
///
/// Example:
///
/// > RELATED-TO:jsmith.part7.19960817T083000.xyzMail@example.com
///
/// [Section 3.8.4.5](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.5)
pub struct RelatedTo {
    value: Uid,
    params: Params,
}

/// This property defines a Uniform Resource Locator (URL) associated with
/// the iCalendar object.
///
/// Example:
///
/// > URL:http://example.com/pub/busy/jpublic-01.ifb
///
/// [Section 3.8.4.6](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.6)
pub struct UniformResourceLocator {
    value: Uri,
    params: Params,
}

/// This property defines the persistent, globally unique identifier for the
/// calendar component.  The UID itself MUST be a globally unique identifier.
/// The generator of the identifier MUST guarantee that the identifier is
/// unique.  There are several algorithms that can be used to accomplish
/// this.  The identifier is recommended to be the identical syntax to the
/// [RFC5322] `Message-ID` header field.  In this case, the identifier would
/// be an email message identifier prepended with the "UID:" label.
///
/// Example:
///
/// > UID:19960401T080045Z-4000F192713-0052@example.com
///
/// [Section 3.8.4.7](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.4.7)
pub struct Uid {
    value: Text,
    params: Params,
}

mod params {
    use crate::{
        params::{
            CalendarUserType, CommonName, DataTypes, Delegatees, Delegators,
            DirectoryEntryReference, Language, Member, ParticipationStatus,
            RecurrenceIdentifierRange, RelationshipType, Rsvp, SentBy,
        },
        timezone::TimeZoneIdentifier,
    };

    /// Parameter bundle for [`RelatedTo`].
    #[derive(Default)]
    pub struct RtParams {
        rt: Option<RelationshipType>,
    }

    /// Parameter bundle for [`RecurrenceId`].
    #[derive(Default)]
    pub struct RecurrenceParams {
        data_type: Option<DataTypes>,
        tzid: Option<TimeZoneIdentifier>,
        recurrence: Option<RecurrenceIdentifierRange>,
    }

    /// Parameter bundle for [`Attendee`].
    #[derive(Default)]
    pub struct AttendeeParams {
        language: Option<Language>,
        calendar_user_type: Option<CalendarUserType>,
        member: Option<Member>,
        status: Option<ParticipationStatus>,
        rsvp: Option<Rsvp>,
        deletegatee: Option<Delegatees>,
        delegator: Option<Delegators>,
        sent_by: Option<SentBy>,
        common_name: Option<CommonName>,
        directory: Option<DirectoryEntryReference>,
    }
}
