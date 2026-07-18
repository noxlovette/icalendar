use crate::ast::{ParseResult, split_once};
use crate::params::*;

/// Convenience alias for the parsed param vector
pub type Params = Vec<PropertyParams>;

/// Any property parameter defined in Section 3.2 of RFC 5545.
///
/// A property can have attributes with which it is associated.
/// These "property parameters" contain meta-information about the
/// property or the property value.  Property parameters are specified
/// on a semicolon-separated list on the line of the property.
/// Property parameter values that contain the COLON, SEMICOLON, or
/// COMMA character separators MUST be specified as quoted-string
/// values.  Property values that do not contain any character
/// delimiters do not need to be quoted but may be.  Some property
/// parameter values are defined using a list of values.  In these
/// cases, the parameter value is defined as a COMMA-separated list of
/// values.
///
/// Example:
///
/// > DTSTART;TZID=America/New_York;VALUE=DATE-TIME:19980119T020000
///
/// [Section 3.2](https://datatracker.ietf.org/doc/html/rfc5545#section-3.2)
enum PropertyParams {
    /// Alternate text representation URI (`ALTREP`). §3.2.1
    Altrep(Altrep),
    /// Common name for a calendar user (`CN`). §3.2.2
    CommonName(CommonName),
    /// Calendar user type (`CUTYPE`). §3.2.3
    CalendarUserType(CalendarUserType),
    /// Delegators — who delegated to this attendee (`DELEGATED-FROM`). §3.2.4
    Delegators(Delegators),
    /// Delegatees — to whom this attendee delegated (`DELEGATED-TO`). §3.2.5
    Delegatees(Delegatees),
    /// Directory entry reference URI (`DIR`). §3.2.6
    DirectoryEntryReference(DirectoryEntryReference),
    /// Inline encoding scheme (`ENCODING`). §3.2.7
    Encoding(Encoding),
    /// MIME media type of a referenced object (`FMTTYPE`). §3.2.8
    Fmttype(Fmttype),
    /// Free/busy time type (`FBTYPE`). §3.2.9
    Fbtype(Fbtype),
    /// Language of property text (`LANGUAGE`). §3.2.10
    Language(Language),
    /// Group/list membership addresses (`MEMBER`). §3.2.11
    Member(Member),
    /// Participation status (`PARTSTAT`). §3.2.12
    ParticipationStatus(ParticipationStatus),
    /// Recurrence identifier range (`RANGE`). §3.2.13
    RecurrenceIdentifierRange(RecurrenceIdentifierRange),
    /// Alarm trigger relationship (`RELATED`). §3.2.14
    AlarmTriggerRelationship(AlarmTriggerRelationship),
    /// Hierarchical relationship type (`RELTYPE`). §3.2.15
    RelationshipType(RelationshipType),
    /// Participation role (`ROLE`). §3.2.16
    ParticipationRole(ParticipationRole),
    /// Reply requested flag (`RSVP`). §3.2.17
    Rsvp(Rsvp),
    /// Acting-on-behalf-of address (`SENT-BY`). §3.2.18
    SentBy(SentBy),
    /// Time zone identifier (`TZID`). §3.2.19
    TimeZoneIdentifier(TimeZoneIdentifier),
    /// Explicit value type (`VALUE`). §3.2.20
    ValueDataType(ValueDataType),
    /// Non-standard `X-`-prefixed parameter.
    XName {
        name: crate::values::Text,
        value: crate::values::Text,
    },
    /// IANA-registered parameter not listed above.
    Iana {
        name: crate::values::Text,
        value: crate::values::Text,
    },
}

impl PropertyParams {
    fn parse(b: &[u8]) -> ParseResult<Vec<Self>> {
        let mut out = Vec::new();
        // need to split by SEMICOLON BUT a SEMICOLON that is NOT in DOUBLE QUOTES
        for s in b.split(|b| *b == b';') {
            let (n, v) = split_once(s, b'=')?;
            let res = match n {
                b"ALTREP" => Self::Altrep(v.try_into()?),
                b"CN" => Self::CommonName(v.try_into()?),
                b"CUTYPE" => Self::CalendarUserType(v.try_into()?),
                b"DELEGATED-FROM" => Self::Delegators(v.try_into()?),
                b"DELEGATED-TO" => Self::Delegatees(v.try_into()?),
                b"DIR" => Self::DirectoryEntryReference(v.try_into()?),
                b"ENCODING" => Self::Encoding(v.try_into()?),
                b"FMTTYPE" => Self::Fmttype(v.try_into()?),
                b"FBTYPE" => Self::Fbtype(v.try_into()?),
                b"LANGUAGE" => Self::Language(v.try_into()?),
                b"MEMBER" => Self::Member(v.try_into()?),
                b"PARTSTAT" => Self::ParticipationStatus(v.try_into()?),
                b"RANGE" => Self::RecurrenceIdentifierRange(v.try_into()?),
                b"RELATED" => Self::AlarmTriggerRelationship(v.try_into()?),
                b"RELTYPE" => Self::RelationshipType(v.try_into()?),
                b"ROLE" => Self::ParticipationRole(v.try_into()?),
                b"RSVP" => Self::Rsvp(v.try_into()?),
                b"SENT-BY" => Self::SentBy(v.try_into()?),
                b"TZID" => Self::TimeZoneIdentifier(v.try_into()?),
                b"VALUE" => Self::ValueDataType(v.try_into()?),
                x => {
                    if x.to_ascii_uppercase().starts_with(b"X-") {
                        Self::XName {
                            name: x.try_into()?,
                            value: v.try_into()?,
                        }
                    } else {
                        Self::Iana {
                            name: x.try_into()?,
                            value: v.try_into()?,
                        }
                    }
                }
            };
            out.push(res);
        }
        Ok(out)
    }
}
