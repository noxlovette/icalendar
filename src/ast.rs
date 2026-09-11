pub mod lexer;
pub mod parser;
mod token;
mod validator;
use parser::{ParseError, ParseResult};

use crate::{
    Calendar,
    calendar::Component as CalComponent,
    components::{
        alarm::Alarm,
        event::Event,
        free_busy::FreeBusy,
        journal::Journal,
        timezone::{Timezone, TzProp},
        todo::Todo,
    },
    properties::*,
};

/// Splits a Bytes vector by given pattern
pub(crate) fn split_once(b: &[u8], needle: u8) -> ParseResult<(&[u8], &[u8])> {
    b.iter()
        .position(|b| *b == needle)
        .map(|pos| (&b[..pos], &b[pos + 1..]))
        .ok_or(ParseError::Parameter {
            expected: needle.to_string(),
            received: std::str::from_utf8(b).ok().map(|s| s.into()),
        })
}

/// [Case-insensitively](https://datatracker.ietf.org/doc/html/rfc5545#section-3.5) matches the name to a given pattern
pub(crate) fn match_name(b: &[u8], pat: &[u8]) -> ParseResult<()> {
    if b.to_ascii_uppercase() != pat {
        Err(ParseError::Parameter {
            expected: std::str::from_utf8(pat)
                .unwrap_or("Unknown pattern")
                .into(),
            received: std::str::from_utf8(b).ok().map(|s| s.into()),
        })
    } else {
        Ok(())
    }
}

/// Finds the byte offset of the first unquoted occurrence of `needle` in
/// `b`. A `needle` byte between two DQUOTE (`"`) characters doesn't count,
/// since content lines and parameter values MAY contain the character
/// they're normally split on once quoted (e.g. a `;` inside an `ALTREP`
/// URI, or the `:` that starts the value appearing inside a quoted
/// parameter value).
pub(crate) fn find_unquoted(b: &[u8], needle: u8) -> Option<usize> {
    let mut in_quotes = false;
    for (i, &byte) in b.iter().enumerate() {
        match byte {
            b'"' => in_quotes = !in_quotes,
            b if b == needle && !in_quotes => return Some(i),
            _ => {}
        }
    }
    None
}

/// Checks if a given value is in quotes and returns that value with the quotes
/// stripped
pub(crate) fn strip_quoted_string(v: &[u8]) -> ParseResult<&[u8]> {
    let needle = &[b'"'];

    v.strip_prefix(needle)
        .and_then(|s| s.strip_suffix(needle))
        .ok_or(ParseError::QuotedString)
}

/// The calendar component carried by an [`crate::ICalendar`] object.
///
/// Each variant corresponds to a component type defined in RFC 5545 Section
/// 3.6.
#[derive(Debug)]
enum Component {
    /// A scheduled event (`VEVENT`).
    Event(EventBuilder),
    /// A to-do task (`VTODO`).
    Todo(TodoBuilder),
    /// A journal entry (`VJOURNAL`).
    Journal(JournalBuilder),
    /// Free/busy time information (`VFREEBUSY`).
    FreeBusy(FreeBusyBuilder),
    /// Time zone definition (`VTIMEZONE`).
    Timezone(TimezoneBuilder),
}

impl From<EventBuilder> for Component {
    fn from(value: EventBuilder) -> Self {
        Self::Event(value)
    }
}
impl From<TodoBuilder> for Component {
    fn from(value: TodoBuilder) -> Self {
        Self::Todo(value)
    }
}

impl From<JournalBuilder> for Component {
    fn from(value: JournalBuilder) -> Self {
        Self::Journal(value)
    }
}

impl From<FreeBusyBuilder> for Component {
    fn from(value: FreeBusyBuilder) -> Self {
        Self::FreeBusy(value)
    }
}

impl From<TimezoneBuilder> for Component {
    fn from(value: TimezoneBuilder) -> Self {
        Self::Timezone(value)
    }
}

impl Component {
    /// Routes one already-parsed [`Property`] into the matching builder's
    /// own fields. What's legal for a given component is decided entirely
    /// by that component's own [`PropertyIngest`] impl — this is just the
    /// dispatch from "which component" to "which builder".
    fn ingest(&mut self, p: Property) -> ParseResult<()> {
        match self {
            Self::Event(b) => b.ingest(p),
            Self::Todo(b) => b.ingest(p),
            Self::Journal(b) => b.ingest(p),
            Self::FreeBusy(b) => b.ingest(p),
            Self::Timezone(b) => b.ingest(p),
        }
    }

    /// Routes a fully-parsed `VALARM` sub-component (see
    /// [`crate::ast::parser::Parser::alarm`]) into a builder that's allowed
    /// to contain one — `VEVENT`/`VTODO` only (RFC 5545 §3.6.1, §3.6.2).
    /// No other component can legally contain a `VALARM`, so this is a
    /// plain match rather than a trait every builder has to implement.
    fn ingest_alarm(&mut self, alarm: AlarmBuilder) -> ParseResult<()> {
        match self {
            Self::Event(b) => Ok(b.alarms.push(alarm)),
            Self::Todo(b) => Ok(b.alarms.push(alarm)),
            _ => Err(ParseError::UnexpectedComponent("VALARM")),
        }
    }

    /// Routes a fully-parsed `STANDARD`/`DAYLIGHT` sub-component (see
    /// [`crate::ast::parser::Parser::tz_observance`]) into the matching
    /// observance list — `VTIMEZONE` only (RFC 5545 §3.6.5).
    fn ingest_tz_observance(
        &mut self,
        kind: TzObservanceKind,
        tz_prop: TzPropBuilder,
    ) -> ParseResult<()> {
        match self {
            Self::Timezone(b) => Ok(match kind {
                TzObservanceKind::Standard => b.standardc.push(tz_prop),
                TzObservanceKind::Daylight => b.daylightc.push(tz_prop),
            }),
            _ => Err(ParseError::UnexpectedComponent("STANDARD/DAYLIGHT")),
        }
    }

    /// Builds this builder into its finished [`CalComponent`], validating
    /// whatever couldn't be checked property-by-property during ingest.
    /// `has_method` is only consulted by [`EventBuilder::build`] (RFC 5545
    /// §3.6.1's `DTSTART`/`METHOD` interaction) — every other component
    /// ignores it.
    fn build(self, has_method: bool) -> Result<CalComponent, CalendarError> {
        Ok(match self {
            Self::Event(b) => CalComponent::Event(b.build(has_method)?),
            Self::Todo(b) => CalComponent::Todo(b.build()?),
            Self::Journal(b) => CalComponent::Journal(b.build()?),
            Self::FreeBusy(b) => CalComponent::FreeBusy(b.build()?),
            Self::Timezone(b) => CalComponent::Timezone(b.build()?),
        })
    }
}

/// Routes one already-parsed [`Property`] into a component builder's own
/// fields. Implemented once per builder, each owning the decision of what's
/// legal for its own component type (RFC 5545 §3.6) — a name absent from a
/// given impl's `match` isn't valid on that component and is an error, the
/// same way [`Property::parse`] owns "what does this keyword mean" instead
/// of the parser.
trait PropertyIngest {
    fn ingest(&mut self, p: Property) -> ParseResult<()>;
}

/// Assigns `value` into a singleton property slot, or reports the RFC 5545
/// violation of the same property occurring twice in one component.
fn set_once<T>(
    slot: &mut Option<T>,
    value: T,
    name: &'static str,
) -> ParseResult<()> {
    if slot.is_some() {
        return Err(ParseError::DuplicateProperty(name));
    }
    *slot = Some(value);
    Ok(())
}

/// A calendar property, wrapping every property type defined in
/// [`crate::properties`].
#[derive(Debug)]
pub(crate) enum Property {
    /// `CALSCALE` ([`CalendarScale`]).
    CalendarScale(CalendarScale),
    /// `METHOD` ([`Method`]).
    Method(Method),
    /// `PRODID` ([`ProductIdentifier`]).
    ProductIdentifier(ProductIdentifier),
    /// `VERSION` ([`Version`]).
    Version(Version),
    /// `ACTION` ([`Action`]).
    Action(Action),
    /// `REPEAT` ([`Repeat`]).
    Repeat(Repeat),
    /// `TRIGGER` ([`Trigger`]).
    Trigger(Trigger),
    /// `CREATED` ([`DateTimeCreated`]).
    DateTimeCreated(DateTimeCreated),
    /// `DTSTAMP` ([`DateTimeStamp`]).
    DateTimeStamp(DateTimeStamp),
    /// `LAST-MODIFIED` ([`LastModified`]).
    LastModified(LastModified),
    /// `SEQUENCE` ([`Sequence`]).
    Sequence(Sequence),
    /// `COMPLETED` ([`Completed`]).
    Completed(Completed),
    /// `DTEND` ([`DateTimeEnd`]).
    DateTimeEnd(DateTimeEnd),
    /// `DUE` ([`DateTimeDue`]).
    DateTimeDue(DateTimeDue),
    /// `DTSTART` ([`DateTimeStart`]).
    DateTimeStart(DateTimeStart),
    /// `DURATION` ([`Duration`]).
    Duration(Duration),
    /// `FREEBUSY` ([`FreeBusyTime`]).
    FreeBusyTime(FreeBusyTime),
    /// `TRANSP` ([`TimeTransparency`]).
    TimeTransparency(TimeTransparency),
    /// `REQUEST-STATUS` ([`RequestStatus`]).
    RequestStatus(RequestStatus),
    /// `ATTACH` ([`Attachment`]).
    Attachment(Attachment),
    /// `CATEGORIES` ([`Categories`]).
    Categories(Categories),
    /// `CLASS` ([`Classification`]).
    Classification(Classification),
    /// `COMMENT` ([`Comment`]).
    Comment(Comment),
    /// `DESCRIPTION` ([`Description`]).
    Description(Description),
    /// `GEO` ([`Geo`]).
    Geo(Geo),
    /// `LOCATION` ([`Location`]).
    Location(Location),
    /// `PERCENT-COMPLETE` ([`PercentComplete`]).
    PercentComplete(PercentComplete),
    /// `PRIORITY` ([`Priority`]).
    Priority(Priority),
    /// `RESOURCES` ([`Resources`]).
    Resources(Resources),
    /// `STATUS` ([`Status`]).
    Status(Status),
    /// `SUMMARY` ([`Summary`]).
    Summary(Summary),
    /// `ATTENDEE` ([`Attendee`]).
    Attendee(Attendee),
    /// `CONTACT` ([`Contact`]).
    Contact(Contact),
    /// `ORGANIZER` ([`Organizer`]).
    Organizer(Organizer),
    /// `RECURRENCE-ID` ([`RecurrenceId`]).
    RecurrenceId(RecurrenceId),
    /// `RELATED-TO` ([`RelatedTo`]).
    RelatedTo(RelatedTo),
    /// `URL` ([`UniformResourceLocator`]).
    UniformResourceLocator(UniformResourceLocator),
    /// `UID` ([`Uid`]).
    Uid(Uid),
    /// `TZID` ([`TimeZoneIdentifier`]).
    TimeZoneIdentifier(TimeZoneIdentifier),
    /// `TZNAME` ([`TimeZoneName`]).
    TimeZoneName(TimeZoneName),
    /// `TZOFFSETFROM` ([`TimeZoneOffsetFrom`]).
    TimeZoneOffsetFrom(TimeZoneOffsetFrom),
    /// `TZOFFSETTO` ([`TimeZoneOffsetTo`]).
    TimeZoneOffsetTo(TimeZoneOffsetTo),
    /// `TZURL` ([`TimeZoneUrl`]).
    TimeZoneUrl(TimeZoneUrl),
    /// `EXDATE` ([`ExceptionDateTimes`]).
    ExceptionDateTimes(ExceptionDateTimes),
    /// `RDATE` ([`RecurrenceDateTimes`]).
    RecurrenceDateTimes(RecurrenceDateTimes),
    /// `RRULE` ([`RRule`]).
    RRule(RRule),
    /// A non-standard, experimental property (`X-` prefixed).
    Xprop(Xprop),
    /// An IANA-registered property.
    Iana(Iana),
}

impl From<CalendarScale> for Property {
    fn from(value: CalendarScale) -> Self {
        Self::CalendarScale(value)
    }
}
impl From<Method> for Property {
    fn from(value: Method) -> Self {
        Self::Method(value)
    }
}
impl From<ProductIdentifier> for Property {
    fn from(value: ProductIdentifier) -> Self {
        Self::ProductIdentifier(value)
    }
}
impl From<Version> for Property {
    fn from(value: Version) -> Self {
        Self::Version(value)
    }
}
impl From<Action> for Property {
    fn from(value: Action) -> Self {
        Self::Action(value)
    }
}
impl From<Repeat> for Property {
    fn from(value: Repeat) -> Self {
        Self::Repeat(value)
    }
}
impl From<Trigger> for Property {
    fn from(value: Trigger) -> Self {
        Self::Trigger(value)
    }
}
impl From<DateTimeCreated> for Property {
    fn from(value: DateTimeCreated) -> Self {
        Self::DateTimeCreated(value)
    }
}
impl From<DateTimeStamp> for Property {
    fn from(value: DateTimeStamp) -> Self {
        Self::DateTimeStamp(value)
    }
}
impl From<LastModified> for Property {
    fn from(value: LastModified) -> Self {
        Self::LastModified(value)
    }
}
impl From<Sequence> for Property {
    fn from(value: Sequence) -> Self {
        Self::Sequence(value)
    }
}
impl From<Completed> for Property {
    fn from(value: Completed) -> Self {
        Self::Completed(value)
    }
}
impl From<DateTimeEnd> for Property {
    fn from(value: DateTimeEnd) -> Self {
        Self::DateTimeEnd(value)
    }
}
impl From<DateTimeDue> for Property {
    fn from(value: DateTimeDue) -> Self {
        Self::DateTimeDue(value)
    }
}
impl From<DateTimeStart> for Property {
    fn from(value: DateTimeStart) -> Self {
        Self::DateTimeStart(value)
    }
}
impl From<Duration> for Property {
    fn from(value: Duration) -> Self {
        Self::Duration(value)
    }
}
impl From<FreeBusyTime> for Property {
    fn from(value: FreeBusyTime) -> Self {
        Self::FreeBusyTime(value)
    }
}
impl From<TimeTransparency> for Property {
    fn from(value: TimeTransparency) -> Self {
        Self::TimeTransparency(value)
    }
}
impl From<RequestStatus> for Property {
    fn from(value: RequestStatus) -> Self {
        Self::RequestStatus(value)
    }
}
impl From<Attachment> for Property {
    fn from(value: Attachment) -> Self {
        Self::Attachment(value)
    }
}
impl From<Categories> for Property {
    fn from(value: Categories) -> Self {
        Self::Categories(value)
    }
}
impl From<Classification> for Property {
    fn from(value: Classification) -> Self {
        Self::Classification(value)
    }
}
impl From<Comment> for Property {
    fn from(value: Comment) -> Self {
        Self::Comment(value)
    }
}
impl From<Description> for Property {
    fn from(value: Description) -> Self {
        Self::Description(value)
    }
}
impl From<Geo> for Property {
    fn from(value: Geo) -> Self {
        Self::Geo(value)
    }
}
impl From<Location> for Property {
    fn from(value: Location) -> Self {
        Self::Location(value)
    }
}
impl From<PercentComplete> for Property {
    fn from(value: PercentComplete) -> Self {
        Self::PercentComplete(value)
    }
}
impl From<Priority> for Property {
    fn from(value: Priority) -> Self {
        Self::Priority(value)
    }
}
impl From<Resources> for Property {
    fn from(value: Resources) -> Self {
        Self::Resources(value)
    }
}
impl From<Status> for Property {
    fn from(value: Status) -> Self {
        Self::Status(value)
    }
}
impl From<Summary> for Property {
    fn from(value: Summary) -> Self {
        Self::Summary(value)
    }
}
impl From<Attendee> for Property {
    fn from(value: Attendee) -> Self {
        Self::Attendee(value)
    }
}
impl From<Contact> for Property {
    fn from(value: Contact) -> Self {
        Self::Contact(value)
    }
}
impl From<Organizer> for Property {
    fn from(value: Organizer) -> Self {
        Self::Organizer(value)
    }
}
impl From<RecurrenceId> for Property {
    fn from(value: RecurrenceId) -> Self {
        Self::RecurrenceId(value)
    }
}
impl From<RelatedTo> for Property {
    fn from(value: RelatedTo) -> Self {
        Self::RelatedTo(value)
    }
}
impl From<UniformResourceLocator> for Property {
    fn from(value: UniformResourceLocator) -> Self {
        Self::UniformResourceLocator(value)
    }
}
impl From<Uid> for Property {
    fn from(value: Uid) -> Self {
        Self::Uid(value)
    }
}
impl From<TimeZoneIdentifier> for Property {
    fn from(value: TimeZoneIdentifier) -> Self {
        Self::TimeZoneIdentifier(value)
    }
}
impl From<TimeZoneName> for Property {
    fn from(value: TimeZoneName) -> Self {
        Self::TimeZoneName(value)
    }
}
impl From<TimeZoneOffsetFrom> for Property {
    fn from(value: TimeZoneOffsetFrom) -> Self {
        Self::TimeZoneOffsetFrom(value)
    }
}
impl From<TimeZoneOffsetTo> for Property {
    fn from(value: TimeZoneOffsetTo) -> Self {
        Self::TimeZoneOffsetTo(value)
    }
}
impl From<TimeZoneUrl> for Property {
    fn from(value: TimeZoneUrl) -> Self {
        Self::TimeZoneUrl(value)
    }
}
impl From<ExceptionDateTimes> for Property {
    fn from(value: ExceptionDateTimes) -> Self {
        Self::ExceptionDateTimes(value)
    }
}
impl From<RecurrenceDateTimes> for Property {
    fn from(value: RecurrenceDateTimes) -> Self {
        Self::RecurrenceDateTimes(value)
    }
}
impl From<RRule> for Property {
    fn from(value: RRule) -> Self {
        Self::RRule(value)
    }
}
impl From<Xprop> for Property {
    fn from(value: Xprop) -> Self {
        Self::Xprop(value)
    }
}
impl From<Iana> for Property {
    fn from(value: Iana) -> Self {
        Self::Iana(value)
    }
}

/// Parses a property's raw, unparsed remainder (`*(";" param) ":" value`,
/// exactly what a [`TokenType::Property`](super::ast::token::TokenType::Property)
/// token's `literal()` carries) into the matching [`Property`] variant.
/// Backs [`Property::parse`]'s dispatch table.
type PropertyParser = fn(&[u8]) -> ParseResult<Property>;

/// Name -> parser dispatch table for every property RFC 5545 defines by
/// keyword (§3.7, §3.8), used by [`Property::parse`]. This is the single
/// place that knows "PRODID means a `ProductIdentifier`" — replacing what
/// used to be a `TokenType` keyword classified by the lexer itself. Built
/// with [`phf`] (the same mechanism the old lexer used for its `KEYWORDS`
/// map) so the lookup stays O(1) — a compile-time perfect hash, not a
/// `match` over byte-string patterns (which codegens as a comparison chain,
/// not a jump table).
static PROPERTY_DISPATCH: phf::Map<&'static [u8], PropertyParser> = phf::phf_map! {
    b"CALSCALE" => |v| CalendarScale::try_from(v).map(Into::into),
    b"METHOD" => |v| Method::try_from(v).map(Into::into),
    b"PRODID" => |v| ProductIdentifier::try_from(v).map(Into::into),
    b"VERSION" => |v| Version::try_from(v).map(Into::into),
    b"ACTION" => |v| Action::try_from(v).map(Into::into),
    b"REPEAT" => |v| Repeat::try_from(v).map(Into::into),
    b"TRIGGER" => |v| Trigger::try_from(v).map(Into::into),
    b"CREATED" => |v| DateTimeCreated::try_from(v).map(Into::into),
    b"DTSTAMP" => |v| DateTimeStamp::try_from(v).map(Into::into),
    b"LAST-MODIFIED" => |v| LastModified::try_from(v).map(Into::into),
    b"SEQUENCE" => |v| Sequence::try_from(v).map(Into::into),
    b"COMPLETED" => |v| Completed::try_from(v).map(Into::into),
    b"DTEND" => |v| DateTimeEnd::try_from(v).map(Into::into),
    b"DUE" => |v| DateTimeDue::try_from(v).map(Into::into),
    b"DTSTART" => |v| DateTimeStart::try_from(v).map(Into::into),
    b"DURATION" => |v| Duration::try_from(v).map(Into::into),
    b"FREEBUSY" => |v| FreeBusyTime::try_from(v).map(Into::into),
    b"TRANSP" => |v| TimeTransparency::try_from(v).map(Into::into),
    b"REQUEST-STATUS" => |v| RequestStatus::try_from(v).map(Into::into),
    b"ATTACH" => |v| Attachment::try_from(v).map(Into::into),
    b"CATEGORIES" => |v| Categories::try_from(v).map(Into::into),
    b"CLASS" => |v| Classification::try_from(v).map(Into::into),
    b"COMMENT" => |v| Comment::try_from(v).map(Into::into),
    b"DESCRIPTION" => |v| Description::try_from(v).map(Into::into),
    b"GEO" => |v| Geo::try_from(v).map(Into::into),
    b"LOCATION" => |v| Location::try_from(v).map(Into::into),
    b"PERCENT-COMPLETE" => |v| PercentComplete::try_from(v).map(Into::into),
    b"PRIORITY" => |v| Priority::try_from(v).map(Into::into),
    b"RESOURCES" => |v| Resources::try_from(v).map(Into::into),
    b"STATUS" => |v| Status::try_from(v).map(Into::into),
    b"SUMMARY" => |v| Summary::try_from(v).map(Into::into),
    b"ATTENDEE" => |v| Attendee::try_from(v).map(Into::into),
    b"CONTACT" => |v| Contact::try_from(v).map(Into::into),
    b"ORGANIZER" => |v| Organizer::try_from(v).map(Into::into),
    b"RECURRENCE-ID" => |v| RecurrenceId::try_from(v).map(Into::into),
    b"RELATED-TO" => |v| RelatedTo::try_from(v).map(Into::into),
    b"URL" => |v| UniformResourceLocator::try_from(v).map(Into::into),
    b"UID" => |v| Uid::try_from(v).map(Into::into),
    b"TZID" => |v| TimeZoneIdentifier::try_from(v).map(Into::into),
    b"TZNAME" => |v| TimeZoneName::try_from(v).map(Into::into),
    b"TZOFFSETFROM" => |v| TimeZoneOffsetFrom::try_from(v).map(Into::into),
    b"TZOFFSETTO" => |v| TimeZoneOffsetTo::try_from(v).map(Into::into),
    b"TZURL" => |v| TimeZoneUrl::try_from(v).map(Into::into),
    b"EXDATE" => |v| ExceptionDateTimes::try_from(v).map(Into::into),
    b"RDATE" => |v| RecurrenceDateTimes::try_from(v).map(Into::into),
    b"RRULE" => |v| RRule::try_from(v).map(Into::into),
};

impl Property {
    /// Parses a [`TokenType::Property`](super::ast::token::TokenType::Property)
    /// token's name and raw remainder (`*(";" param) ":" value`) into the
    /// matching [`Property`] variant, dispatching on the token's upper-cased
    /// name (`lexeme()`) via [`PROPERTY_DISPATCH`]. A name absent from the
    /// table isn't an error — `X-`/IANA extension properties are open-ended
    /// by design
    /// ([Section 3.8.8](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.8))
    /// — it just falls back to [`Xprop`]/[`Iana`].
    pub(crate) fn parse(
        name: &[u8],
        remainder: &[u8],
    ) -> ParseResult<Property> {
        if let Some(parse) = PROPERTY_DISPATCH.get(name) {
            parse(remainder)
        } else if name.starts_with(b"X-") {
            Xprop::try_from(remainder).map(Into::into)
        } else {
            Iana::try_from(remainder).map(Into::into)
        }
    }
}

#[derive(Default, Debug)]
struct CalendarBuilder {
    prodid: Option<ProductIdentifier>,
    version: Option<Version>,
    calscale: Option<CalendarScale>,
    method: Option<Method>,
    xprop: Vec<Xprop>,
    iana: Vec<Iana>,
    components: Vec<Component>,
}

impl CalendarBuilder {
    /// creates a new cal builder
    fn new() -> Self {
        Self::default()
    }

    fn build(self) -> Result<Calendar, CalendarError> {
        let prodid = self
            .prodid
            .ok_or(CalendarError::MissingField("PRODID"))?;
        let version = self
            .version
            .ok_or(CalendarError::MissingField("VERSION"))?;
        if self.components.is_empty() {
            return Err(CalendarError::RequiresAtLeastOne(
                "VCALENDAR",
                "calendar component",
            ));
        }
        // DTSTART is only REQUIRED on a VEVENT when METHOD is absent (RFC
        // 5545 §3.6.1) — every component needs to know this to validate
        // itself, so it's threaded down rather than re-checked per-event.
        let has_method = self.method.is_some();
        let components = self
            .components
            .into_iter()
            .map(|c| c.build(has_method))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Calendar {
            prodid,
            version,
            calscale: self.calscale,
            method: self.method,
            xprop: self.xprop,
            iana: self.iana,
            components,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum CalendarError {
    #[error("Missing field: {0}")]
    MissingField(&'static str),

    /// Two properties that RFC 5545 says MUST NOT both appear in the same
    /// component (e.g. `DTEND`/`DURATION`) were both present.
    #[error("{0} and {1} MUST NOT both be specified in the same component")]
    MutuallyExclusive(&'static str, &'static str),

    /// A property that RFC 5545 only allows conditional on another being
    /// present (e.g. `DURATION` requiring `DTSTART` in a `VTODO`) showed up
    /// without it.
    #[error("{0} MUST NOT be specified without {1}")]
    Requires(&'static str, &'static str),

    /// Two properties that RFC 5545 says MUST occur together or not at all
    /// (e.g. `DURATION`/`REPEAT` in a `VALARM`) had only one present.
    #[error("{0} and {1} MUST occur together or not at all")]
    RequiresTogether(&'static str, &'static str),

    /// A component/property required at least one of some other property,
    /// but none were present (e.g. `VALARM` with `ACTION:EMAIL` requires at
    /// least one `ATTENDEE`).
    #[error("{0} requires at least one {1}")]
    RequiresAtLeastOne(&'static str, &'static str),

    /// A property RFC 5545 disallows for a particular alternative grammar
    /// showed up anyway (e.g. `ATTACH` in a `VALARM` with `ACTION:DISPLAY`).
    #[error("{0} MUST NOT be specified when {1}")]
    NotAllowed(&'static str, &'static str),
}

#[derive(Debug, Default)]
struct EventBuilder {
    dtstamp: Option<DateTimeStamp>,
    uid: Option<Uid>,
    /// The following is REQUIRED if the component
    /// appears in an iCalendar object that doesn't
    /// specify the "METHOD" property; otherwise, it
    /// is OPTIONAL; in any case, it MUST NOT occur
    /// more than once.
    dtstart: Option<DateTimeStart>,
    class: Option<Classification>,
    created: Option<DateTimeCreated>,
    description: Option<Description>,
    geo: Option<Geo>,
    last_mod: Option<LastModified>,
    location: Option<Location>,
    organizer: Option<Organizer>,
    priority: Option<Priority>,
    seq: Option<Sequence>,
    status: Option<Status>,
    summary: Option<Summary>,
    transp: Option<TimeTransparency>,
    url: Option<UniformResourceLocator>,
    recurid: Option<RecurrenceId>,
    rrule: Option<RRule>,
    dtend: Option<DateTimeEnd>,
    duration: Option<Duration>,
    attach: Vec<Attachment>,
    attendee: Vec<Attendee>,
    categories: Vec<Categories>,
    comment: Vec<Comment>,
    contact: Vec<Contact>,
    exdate: Vec<ExceptionDateTimes>,
    rstatus: Vec<RequestStatus>,
    related: Vec<RelatedTo>,
    resources: Vec<Resources>,
    rdate: Vec<RecurrenceDateTimes>,
    xprop: Vec<Xprop>,
    iana: Vec<Iana>,
    alarms: Vec<AlarmBuilder>,
}

impl EventBuilder {
    fn new() -> Self {
        Self::default()
    }

    /// Validates the cross-field rules RFC 5545 §3.6.1 places on `VEVENT`
    /// and assembles the finished [`Event`]. `has_method` is whether the
    /// enclosing `VCALENDAR` specified a `METHOD` property — `DTSTART` is
    /// only REQUIRED here when it didn't.
    fn build(self, has_method: bool) -> Result<Event, CalendarError> {
        if !has_method && self.dtstart.is_none() {
            return Err(CalendarError::MissingField("DTSTART"));
        }
        if self.dtend.is_some() && self.duration.is_some() {
            return Err(CalendarError::MutuallyExclusive("DTEND", "DURATION"));
        }
        let alarms = self
            .alarms
            .into_iter()
            .map(AlarmBuilder::build)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Event {
            dtstamp: self
                .dtstamp
                .ok_or(CalendarError::MissingField("DTSTAMP"))?,
            uid: self.uid.ok_or(CalendarError::MissingField("UID"))?,
            dtstart: self.dtstart,
            class: self.class,
            created: self.created,
            description: self.description,
            geo: self.geo,
            last_mod: self.last_mod,
            location: self.location,
            organizer: self.organizer,
            priority: self.priority,
            seq: self.seq,
            status: self.status,
            summary: self.summary,
            transp: self.transp,
            url: self.url,
            recurid: self.recurid,
            rrule: self.rrule,
            dtend: self.dtend,
            duration: self.duration,
            attach: self.attach,
            attendee: self.attendee,
            categories: self.categories,
            comment: self.comment,
            contact: self.contact,
            exdate: self.exdate,
            rstatus: self.rstatus,
            related: self.related,
            resources: self.resources,
            rdate: self.rdate,
            xprop: self.xprop,
            iana: self.iana,
            alarms,
        })
    }
}

impl PropertyIngest for EventBuilder {
    fn ingest(&mut self, p: Property) -> ParseResult<()> {
        match p {
            Property::DateTimeStamp(v) => set_once(&mut self.dtstamp, v, "DTSTAMP"),
            Property::Uid(v) => set_once(&mut self.uid, v, "UID"),
            Property::DateTimeStart(v) => set_once(&mut self.dtstart, v, "DTSTART"),
            Property::Classification(v) => set_once(&mut self.class, v, "CLASS"),
            Property::DateTimeCreated(v) => set_once(&mut self.created, v, "CREATED"),
            Property::Description(v) => set_once(&mut self.description, v, "DESCRIPTION"),
            Property::Geo(v) => set_once(&mut self.geo, v, "GEO"),
            Property::LastModified(v) => set_once(&mut self.last_mod, v, "LAST-MODIFIED"),
            Property::Location(v) => set_once(&mut self.location, v, "LOCATION"),
            Property::Organizer(v) => set_once(&mut self.organizer, v, "ORGANIZER"),
            Property::Priority(v) => set_once(&mut self.priority, v, "PRIORITY"),
            Property::Sequence(v) => set_once(&mut self.seq, v, "SEQUENCE"),
            Property::Status(v) => set_once(&mut self.status, v, "STATUS"),
            Property::Summary(v) => set_once(&mut self.summary, v, "SUMMARY"),
            Property::TimeTransparency(v) => set_once(&mut self.transp, v, "TRANSP"),
            Property::UniformResourceLocator(v) => set_once(&mut self.url, v, "URL"),
            Property::RecurrenceId(v) => set_once(&mut self.recurid, v, "RECURRENCE-ID"),
            Property::RRule(v) => set_once(&mut self.rrule, v, "RRULE"),
            // DTEND and DURATION are mutually exclusive within a VEVENT
            // (RFC 5545 §3.6.1) — that's a cross-field rule, checked in
            // `build()` once every property has been seen, not here.
            Property::DateTimeEnd(v) => set_once(&mut self.dtend, v, "DTEND"),
            Property::Duration(v) => set_once(&mut self.duration, v, "DURATION"),
            Property::Attachment(v) => Ok(self.attach.push(v)),
            Property::Attendee(v) => Ok(self.attendee.push(v)),
            Property::Categories(v) => Ok(self.categories.push(v)),
            Property::Comment(v) => Ok(self.comment.push(v)),
            Property::Contact(v) => Ok(self.contact.push(v)),
            Property::ExceptionDateTimes(v) => Ok(self.exdate.push(v)),
            Property::RequestStatus(v) => Ok(self.rstatus.push(v)),
            Property::RelatedTo(v) => Ok(self.related.push(v)),
            Property::Resources(v) => Ok(self.resources.push(v)),
            Property::RecurrenceDateTimes(v) => Ok(self.rdate.push(v)),
            Property::Xprop(v) => Ok(self.xprop.push(v)),
            Property::Iana(v) => Ok(self.iana.push(v)),
            _ => Err(ParseError::UnexpectedProperty),
        }
    }
}

#[derive(Debug, Default)]
struct TodoBuilder {
    dtstamp: Option<DateTimeStamp>,
    uid: Option<Uid>,
    class: Option<Classification>,
    completed: Option<Completed>,
    created: Option<DateTimeCreated>,
    description: Option<Description>,
    dtstart: Option<DateTimeStart>,
    geo: Option<Geo>,
    last_mod: Option<LastModified>,
    location: Option<Location>,
    organizer: Option<Organizer>,
    percent: Option<PercentComplete>,
    priority: Option<Priority>,
    recur_id: Option<RecurrenceId>,
    seq: Option<Sequence>,
    status: Option<Status>,
    summary: Option<Summary>,
    url: Option<UniformResourceLocator>,
    rrule: Option<RRule>,
    due: Option<DateTimeDue>,
    duration: Option<Duration>,
    attach: Vec<Attachment>,
    attendee: Vec<Attendee>,
    categories: Vec<Categories>,
    comment: Vec<Comment>,
    contact: Vec<Contact>,
    exdate: Vec<ExceptionDateTimes>,
    rstatus: Vec<RequestStatus>,
    related: Vec<RelatedTo>,
    resources: Vec<Resources>,
    rdate: Vec<RecurrenceDateTimes>,
    xprop: Vec<Xprop>,
    iana: Vec<Iana>,
    alarms: Vec<AlarmBuilder>,
}

impl TodoBuilder {
    fn new() -> Self {
        Self::default()
    }

    /// Validates the cross-field rules RFC 5545 §3.6.2 places on `VTODO`
    /// and assembles the finished [`Todo`].
    fn build(self) -> Result<Todo, CalendarError> {
        if self.due.is_some() && self.duration.is_some() {
            return Err(CalendarError::MutuallyExclusive("DUE", "DURATION"));
        }
        if self.duration.is_some() && self.dtstart.is_none() {
            return Err(CalendarError::Requires("DURATION", "DTSTART"));
        }
        let alarms = self
            .alarms
            .into_iter()
            .map(AlarmBuilder::build)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Todo {
            dtstamp: self
                .dtstamp
                .ok_or(CalendarError::MissingField("DTSTAMP"))?,
            uid: self.uid.ok_or(CalendarError::MissingField("UID"))?,
            class: self.class,
            completed: self.completed,
            created: self.created,
            description: self.description,
            dtstart: self.dtstart,
            geo: self.geo,
            last_mod: self.last_mod,
            location: self.location,
            organizer: self.organizer,
            percent: self.percent,
            priority: self.priority,
            recur_id: self.recur_id,
            seq: self.seq,
            status: self.status,
            summary: self.summary,
            url: self.url,
            rrule: self.rrule,
            due: self.due,
            duration: self.duration,
            attach: self.attach,
            attendee: self.attendee,
            categories: self.categories,
            comment: self.comment,
            contact: self.contact,
            exdate: self.exdate,
            rstatus: self.rstatus,
            related: self.related,
            resources: self.resources,
            rdate: self.rdate,
            xprop: self.xprop,
            iana: self.iana,
            alarms,
        })
    }
}

impl PropertyIngest for TodoBuilder {
    fn ingest(&mut self, p: Property) -> ParseResult<()> {
        match p {
            Property::DateTimeStamp(v) => set_once(&mut self.dtstamp, v, "DTSTAMP"),
            Property::Uid(v) => set_once(&mut self.uid, v, "UID"),
            Property::Classification(v) => set_once(&mut self.class, v, "CLASS"),
            Property::Completed(v) => set_once(&mut self.completed, v, "COMPLETED"),
            Property::DateTimeCreated(v) => set_once(&mut self.created, v, "CREATED"),
            Property::Description(v) => set_once(&mut self.description, v, "DESCRIPTION"),
            Property::DateTimeStart(v) => set_once(&mut self.dtstart, v, "DTSTART"),
            Property::Geo(v) => set_once(&mut self.geo, v, "GEO"),
            Property::LastModified(v) => set_once(&mut self.last_mod, v, "LAST-MODIFIED"),
            Property::Location(v) => set_once(&mut self.location, v, "LOCATION"),
            Property::Organizer(v) => set_once(&mut self.organizer, v, "ORGANIZER"),
            Property::PercentComplete(v) => set_once(&mut self.percent, v, "PERCENT-COMPLETE"),
            Property::Priority(v) => set_once(&mut self.priority, v, "PRIORITY"),
            Property::RecurrenceId(v) => set_once(&mut self.recur_id, v, "RECURRENCE-ID"),
            Property::Sequence(v) => set_once(&mut self.seq, v, "SEQUENCE"),
            Property::Status(v) => set_once(&mut self.status, v, "STATUS"),
            Property::Summary(v) => set_once(&mut self.summary, v, "SUMMARY"),
            Property::UniformResourceLocator(v) => set_once(&mut self.url, v, "URL"),
            Property::RRule(v) => set_once(&mut self.rrule, v, "RRULE"),
            // DUE and DURATION are mutually exclusive within a VTODO, and
            // DURATION requires DTSTART to also be present (RFC 5545
            // §3.6.2) — cross-field rules, checked in `build()`.
            Property::DateTimeDue(v) => set_once(&mut self.due, v, "DUE"),
            Property::Duration(v) => set_once(&mut self.duration, v, "DURATION"),
            Property::Attachment(v) => Ok(self.attach.push(v)),
            Property::Attendee(v) => Ok(self.attendee.push(v)),
            Property::Categories(v) => Ok(self.categories.push(v)),
            Property::Comment(v) => Ok(self.comment.push(v)),
            Property::Contact(v) => Ok(self.contact.push(v)),
            Property::ExceptionDateTimes(v) => Ok(self.exdate.push(v)),
            Property::RequestStatus(v) => Ok(self.rstatus.push(v)),
            Property::RelatedTo(v) => Ok(self.related.push(v)),
            Property::Resources(v) => Ok(self.resources.push(v)),
            Property::RecurrenceDateTimes(v) => Ok(self.rdate.push(v)),
            Property::Xprop(v) => Ok(self.xprop.push(v)),
            Property::Iana(v) => Ok(self.iana.push(v)),
            _ => Err(ParseError::UnexpectedProperty),
        }
    }
}

/// Builder for `VALARM` (RFC 5545 §3.6.6), nested only inside `VEVENT`/
/// `VTODO` (see [`Component::ingest_alarm`]). RFC 5545 actually splits this
/// into three alternative property sets — `audioprop`/`dispprop`/
/// `emailprop` — selected by `ACTION`'s value (AUDIO/DISPLAY/EMAIL), each
/// with its own required/optional/cardinality rules for `DESCRIPTION`,
/// `SUMMARY`, `ATTENDEE`, and `ATTACH`. Ingest only enforces the union of
/// what's structurally possible across all three (hence `Option` here even
/// for fields a given `ACTION` will require, and `Vec` for `ATTACH` even
/// though only `emailprop` allows more than one) — which alternative
/// actually applies, and whether this alarm satisfies it, is a `build()`-
/// time check once `ACTION` is known.
#[derive(Debug, Default)]
struct AlarmBuilder {
    action: Option<Action>,
    trigger: Option<Trigger>,
    // DURATION and REPEAT are optional but MUST appear together (RFC 5545
    // §3.6.6) — a cross-field rule, checked in `build()`.
    duration: Option<Duration>,
    repeat: Option<Repeat>,
    description: Option<Description>,
    summary: Option<Summary>,
    attendee: Vec<Attendee>,
    attach: Vec<Attachment>,
    xprop: Vec<Xprop>,
    iana: Vec<Iana>,
}

impl AlarmBuilder {
    fn new() -> Self {
        Self::default()
    }

    /// Validates the cross-field rules RFC 5545 §3.6.6 places on `VALARM`
    /// and assembles the finished [`Alarm`]. `ACTION` selects which of the
    /// three alternative grammars (`audioprop`/`dispprop`/`emailprop`)
    /// applies — everything beyond "`ACTION` and `TRIGGER` are both
    /// required" is specific to that alternative.
    fn build(self) -> Result<Alarm, CalendarError> {
        let action = self
            .action
            .ok_or(CalendarError::MissingField("ACTION"))?;
        let trigger = self
            .trigger
            .ok_or(CalendarError::MissingField("TRIGGER"))?;
        if self.duration.is_some() != self.repeat.is_some() {
            return Err(CalendarError::RequiresTogether("DURATION", "REPEAT"));
        }

        match action.kind() {
            ActionEnum::Audio => {
                if self.description.is_some() {
                    return Err(CalendarError::NotAllowed(
                        "DESCRIPTION",
                        "ACTION is AUDIO",
                    ));
                }
                if self.summary.is_some() {
                    return Err(CalendarError::NotAllowed(
                        "SUMMARY",
                        "ACTION is AUDIO",
                    ));
                }
                if !self.attendee.is_empty() {
                    return Err(CalendarError::NotAllowed(
                        "ATTENDEE",
                        "ACTION is AUDIO",
                    ));
                }
                if self.attach.len() > 1 {
                    return Err(CalendarError::NotAllowed(
                        "more than one ATTACH",
                        "ACTION is AUDIO",
                    ));
                }
            }
            ActionEnum::Display => {
                if self.description.is_none() {
                    return Err(CalendarError::MissingField("DESCRIPTION"));
                }
                if self.summary.is_some() {
                    return Err(CalendarError::NotAllowed(
                        "SUMMARY",
                        "ACTION is DISPLAY",
                    ));
                }
                if !self.attendee.is_empty() {
                    return Err(CalendarError::NotAllowed(
                        "ATTENDEE",
                        "ACTION is DISPLAY",
                    ));
                }
                if !self.attach.is_empty() {
                    return Err(CalendarError::NotAllowed(
                        "ATTACH",
                        "ACTION is DISPLAY",
                    ));
                }
            }
            ActionEnum::Email => {
                if self.description.is_none() {
                    return Err(CalendarError::MissingField("DESCRIPTION"));
                }
                if self.summary.is_none() {
                    return Err(CalendarError::MissingField("SUMMARY"));
                }
                if self.attendee.is_empty() {
                    return Err(CalendarError::RequiresAtLeastOne(
                        "ACTION EMAIL",
                        "ATTENDEE",
                    ));
                }
            }
            // IANA/X-name actions aren't defined by RFC 5545 — only the
            // common ACTION/TRIGGER/DURATION+REPEAT rules above apply.
            ActionEnum::Iana(_) | ActionEnum::XName(_) => {}
        }

        Ok(Alarm {
            action,
            trigger,
            duration: self.duration,
            repeat: self.repeat,
            description: self.description,
            summary: self.summary,
            attendee: self.attendee,
            attach: self.attach,
            xprop: self.xprop,
            iana: self.iana,
        })
    }
}

impl PropertyIngest for AlarmBuilder {
    fn ingest(&mut self, p: Property) -> ParseResult<()> {
        match p {
            Property::Action(v) => set_once(&mut self.action, v, "ACTION"),
            Property::Trigger(v) => set_once(&mut self.trigger, v, "TRIGGER"),
            Property::Duration(v) => set_once(&mut self.duration, v, "DURATION"),
            Property::Repeat(v) => set_once(&mut self.repeat, v, "REPEAT"),
            Property::Description(v) => set_once(&mut self.description, v, "DESCRIPTION"),
            Property::Summary(v) => set_once(&mut self.summary, v, "SUMMARY"),
            Property::Attendee(v) => Ok(self.attendee.push(v)),
            Property::Attachment(v) => Ok(self.attach.push(v)),
            Property::Xprop(v) => Ok(self.xprop.push(v)),
            Property::Iana(v) => Ok(self.iana.push(v)),
            _ => Err(ParseError::UnexpectedProperty),
        }
    }
}

impl FreeBusyBuilder {
    fn new() -> Self {
        Self::default()
    }

    /// Validates the cross-field rules RFC 5545 §3.6.4 places on
    /// `VFREEBUSY` and assembles the finished [`FreeBusy`].
    fn build(self) -> Result<FreeBusy, CalendarError> {
        Ok(FreeBusy {
            dtstamp: self
                .dtstamp
                .ok_or(CalendarError::MissingField("DTSTAMP"))?,
            uid: self.uid.ok_or(CalendarError::MissingField("UID"))?,
            contact: self.contact,
            dtstart: self.dtstart,
            dtend: self.dtend,
            organizer: self.organizer,
            url: self.url,
            attendee: self.attendee,
            comment: self.comment,
            freebusy: self.freebusy,
            rstatus: self.rstatus,
            xprop: self.xprop,
            iana: self.iana,
        })
    }
}

impl PropertyIngest for FreeBusyBuilder {
    fn ingest(&mut self, p: Property) -> ParseResult<()> {
        match p {
            Property::DateTimeStamp(v) => set_once(&mut self.dtstamp, v, "DTSTAMP"),
            Property::Uid(v) => set_once(&mut self.uid, v, "UID"),
            Property::Contact(v) => set_once(&mut self.contact, v, "CONTACT"),
            Property::DateTimeStart(v) => set_once(&mut self.dtstart, v, "DTSTART"),
            Property::DateTimeEnd(v) => set_once(&mut self.dtend, v, "DTEND"),
            Property::Organizer(v) => set_once(&mut self.organizer, v, "ORGANIZER"),
            Property::UniformResourceLocator(v) => set_once(&mut self.url, v, "URL"),
            Property::Attendee(v) => Ok(self.attendee.push(v)),
            Property::Comment(v) => Ok(self.comment.push(v)),
            Property::FreeBusyTime(v) => Ok(self.freebusy.push(v)),
            Property::RequestStatus(v) => Ok(self.rstatus.push(v)),
            Property::Xprop(v) => Ok(self.xprop.push(v)),
            Property::Iana(v) => Ok(self.iana.push(v)),
            _ => Err(ParseError::UnexpectedProperty),
        }
    }
}
impl JournalBuilder {
    fn new() -> Self {
        Self::default()
    }

    /// Validates the cross-field rules RFC 5545 §3.6.3 places on
    /// `VJOURNAL` and assembles the finished [`Journal`].
    fn build(self) -> Result<Journal, CalendarError> {
        Ok(Journal {
            dtstamp: self
                .dtstamp
                .ok_or(CalendarError::MissingField("DTSTAMP"))?,
            uid: self.uid.ok_or(CalendarError::MissingField("UID"))?,
            class: self.class,
            created: self.created,
            dtstart: self.dtstart,
            last_mod: self.last_mod,
            organizer: self.organizer,
            recurid: self.recurid,
            seq: self.seq,
            status: self.status,
            summary: self.summary,
            url: self.url,
            rrule: self.rrule,
            attach: self.attach,
            attendee: self.attendee,
            categories: self.categories,
            comment: self.comment,
            contact: self.contact,
            description: self.description,
            exdate: self.exdate,
            related: self.related,
            rdate: self.rdate,
            rstatus: self.rstatus,
            xprop: self.xprop,
            iana: self.iana,
        })
    }
}

impl PropertyIngest for JournalBuilder {
    fn ingest(&mut self, p: Property) -> ParseResult<()> {
        match p {
            Property::DateTimeStamp(v) => set_once(&mut self.dtstamp, v, "DTSTAMP"),
            Property::Uid(v) => set_once(&mut self.uid, v, "UID"),
            Property::Classification(v) => set_once(&mut self.class, v, "CLASS"),
            Property::DateTimeCreated(v) => set_once(&mut self.created, v, "CREATED"),
            Property::DateTimeStart(v) => set_once(&mut self.dtstart, v, "DTSTART"),
            Property::LastModified(v) => set_once(&mut self.last_mod, v, "LAST-MODIFIED"),
            Property::Organizer(v) => set_once(&mut self.organizer, v, "ORGANIZER"),
            Property::RecurrenceId(v) => set_once(&mut self.recurid, v, "RECURRENCE-ID"),
            Property::Sequence(v) => set_once(&mut self.seq, v, "SEQUENCE"),
            Property::Status(v) => set_once(&mut self.status, v, "STATUS"),
            Property::Summary(v) => set_once(&mut self.summary, v, "SUMMARY"),
            Property::UniformResourceLocator(v) => set_once(&mut self.url, v, "URL"),
            Property::RRule(v) => set_once(&mut self.rrule, v, "RRULE"),
            Property::Attachment(v) => Ok(self.attach.push(v)),
            Property::Attendee(v) => Ok(self.attendee.push(v)),
            Property::Categories(v) => Ok(self.categories.push(v)),
            Property::Comment(v) => Ok(self.comment.push(v)),
            Property::Contact(v) => Ok(self.contact.push(v)),
            Property::Description(v) => Ok(self.description.push(v)),
            Property::ExceptionDateTimes(v) => Ok(self.exdate.push(v)),
            Property::RelatedTo(v) => Ok(self.related.push(v)),
            Property::RecurrenceDateTimes(v) => Ok(self.rdate.push(v)),
            Property::RequestStatus(v) => Ok(self.rstatus.push(v)),
            Property::Xprop(v) => Ok(self.xprop.push(v)),
            Property::Iana(v) => Ok(self.iana.push(v)),
            _ => Err(ParseError::UnexpectedProperty),
        }
    }
}
#[derive(Debug, Default)]
struct FreeBusyBuilder {
    dtstamp: Option<DateTimeStamp>,
    uid: Option<Uid>,
    contact: Option<Contact>,
    dtstart: Option<DateTimeStart>,
    dtend: Option<DateTimeEnd>,
    organizer: Option<Organizer>,
    url: Option<UniformResourceLocator>,
    attendee: Vec<Attendee>,
    comment: Vec<Comment>,
    freebusy: Vec<FreeBusyTime>,
    rstatus: Vec<RequestStatus>,
    xprop: Vec<Xprop>,
    iana: Vec<Iana>,
}

#[derive(Debug, Default)]
struct JournalBuilder {
    dtstamp: Option<DateTimeStamp>,
    uid: Option<Uid>,
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
    xprop: Vec<Xprop>,
    iana: Vec<Iana>,
}

/// Builder for `VTIMEZONE` (RFC 5545 §3.6.5). `standardc`/`daylightc` hold
/// its nested `STANDARD`/`DAYLIGHT` sub-components (see
/// [`crate::ast::parser::Parser::tz_observance`]) — `build()` is still the
/// place that enforces "at least one of either" once every property and
/// sub-component has been seen.
#[derive(Debug, Default)]
struct TimezoneBuilder {
    tzid: Option<TimeZoneIdentifier>,
    last_mod: Option<LastModified>,
    tzurl: Option<TimeZoneUrl>,
    xprop: Vec<Xprop>,
    iana: Vec<Iana>,
    standardc: Vec<TzPropBuilder>,
    daylightc: Vec<TzPropBuilder>,
}

impl TimezoneBuilder {
    fn new() -> Self {
        Self::default()
    }

    /// Validates the cross-field rules RFC 5545 §3.6.5 places on
    /// `VTIMEZONE` and assembles the finished [`Timezone`]. At least one
    /// `STANDARD` or `DAYLIGHT` sub-component is required — that can only
    /// be checked once every nested sub-component has been seen, hence
    /// here rather than in `ingest`.
    fn build(self) -> Result<Timezone, CalendarError> {
        if self.standardc.is_empty() && self.daylightc.is_empty() {
            return Err(CalendarError::RequiresAtLeastOne(
                "VTIMEZONE",
                "STANDARD or DAYLIGHT",
            ));
        }
        let standardc = self
            .standardc
            .into_iter()
            .map(TzPropBuilder::build)
            .collect::<Result<Vec<_>, _>>()?;
        let daylightc = self
            .daylightc
            .into_iter()
            .map(TzPropBuilder::build)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Timezone {
            tzid: self.tzid.ok_or(CalendarError::MissingField("TZID"))?,
            last_mod: self.last_mod,
            tz_url: self.tzurl,
            standardc,
            daylightc,
            xprop: self.xprop,
            iana: self.iana,
        })
    }
}

impl PropertyIngest for TimezoneBuilder {
    fn ingest(&mut self, p: Property) -> ParseResult<()> {
        match p {
            Property::TimeZoneIdentifier(v) => set_once(&mut self.tzid, v, "TZID"),
            Property::LastModified(v) => set_once(&mut self.last_mod, v, "LAST-MODIFIED"),
            Property::TimeZoneUrl(v) => set_once(&mut self.tzurl, v, "TZURL"),
            Property::Xprop(v) => Ok(self.xprop.push(v)),
            Property::Iana(v) => Ok(self.iana.push(v)),
            _ => Err(ParseError::UnexpectedProperty),
        }
    }
}

/// Which sub-component produced a given [`TzPropBuilder`] — `STANDARD` and
/// `DAYLIGHT` (RFC 5545 §3.6.5) share an identical `tzprop` property
/// grammar and differ only in which of `VTIMEZONE`'s two observance lists
/// they belong to.
enum TzObservanceKind {
    Standard,
    Daylight,
}

/// Builder for the `tzprop` grammar shared by `STANDARD`/`DAYLIGHT`
/// sub-components (RFC 5545 §3.6.5). Nested only inside `VTIMEZONE`, via
/// [`Component::ingest_tz_observance`].
#[derive(Debug, Default)]
struct TzPropBuilder {
    dtstart: Option<DateTimeStart>,
    tz_offset_to: Option<TimeZoneOffsetTo>,
    tz_offset_from: Option<TimeZoneOffsetFrom>,
    // RRULE is optional but SHOULD NOT occur more than once (RFC 5545
    // §3.6.5) — same "singleton in practice" treatment as elsewhere.
    rrule: Option<RRule>,
    comment: Vec<Comment>,
    rdate: Vec<RecurrenceDateTimes>,
    tzname: Vec<TimeZoneName>,
    xprop: Vec<Xprop>,
    iana: Vec<Iana>,
}

impl TzPropBuilder {
    fn new() -> Self {
        Self::default()
    }

    /// Validates the `tzprop` grammar's required fields (RFC 5545 §3.6.5)
    /// and assembles the finished [`TzProp`].
    fn build(self) -> Result<TzProp, CalendarError> {
        Ok(TzProp {
            dtstart: self
                .dtstart
                .ok_or(CalendarError::MissingField("DTSTART"))?,
            tz_offset_to: self
                .tz_offset_to
                .ok_or(CalendarError::MissingField("TZOFFSETTO"))?,
            tz_offset_from: self
                .tz_offset_from
                .ok_or(CalendarError::MissingField("TZOFFSETFROM"))?,
            rrule: self.rrule,
            comment: self.comment,
            rdate: self.rdate,
            tzname: self.tzname,
            xprop: self.xprop,
            iana: self.iana,
        })
    }
}

impl PropertyIngest for TzPropBuilder {
    fn ingest(&mut self, p: Property) -> ParseResult<()> {
        match p {
            Property::DateTimeStart(v) => set_once(&mut self.dtstart, v, "DTSTART"),
            Property::TimeZoneOffsetTo(v) => {
                set_once(&mut self.tz_offset_to, v, "TZOFFSETTO")
            }
            Property::TimeZoneOffsetFrom(v) => {
                set_once(&mut self.tz_offset_from, v, "TZOFFSETFROM")
            }
            Property::RRule(v) => set_once(&mut self.rrule, v, "RRULE"),
            Property::Comment(v) => Ok(self.comment.push(v)),
            Property::RecurrenceDateTimes(v) => Ok(self.rdate.push(v)),
            Property::TimeZoneName(v) => Ok(self.tzname.push(v)),
            Property::Xprop(v) => Ok(self.xprop.push(v)),
            Property::Iana(v) => Ok(self.iana.push(v)),
            _ => Err(ParseError::UnexpectedProperty),
        }
    }
}

#[cfg(test)]
mod build_tests {
    use super::*;

    fn prop(name: &[u8], remainder: &[u8]) -> Property {
        Property::parse(name, remainder).unwrap()
    }

    fn minimal_event() -> EventBuilder {
        let mut b = EventBuilder::new();
        b.ingest(prop(b"DTSTAMP", b":19970901T130000Z")).unwrap();
        b.ingest(prop(b"UID", b":123@example.com")).unwrap();
        b.ingest(prop(b"DTSTART", b":19970903T163000Z")).unwrap();
        b
    }

    #[test]
    fn event_requires_dtstamp() {
        let mut b = minimal_event();
        b.dtstamp = None;
        assert!(matches!(
            b.build(true),
            Err(CalendarError::MissingField("DTSTAMP"))
        ));
    }

    #[test]
    fn event_requires_uid() {
        let mut b = minimal_event();
        b.uid = None;
        assert!(matches!(
            b.build(true),
            Err(CalendarError::MissingField("UID"))
        ));
    }

    #[test]
    fn event_dtstart_required_unless_method_present() {
        let mut b = minimal_event();
        b.dtstart = None;
        assert!(matches!(
            b.build(false),
            Err(CalendarError::MissingField("DTSTART"))
        ));
        assert!(minimal_event().build(true).is_ok());
    }

    #[test]
    fn event_dtend_and_duration_are_mutually_exclusive() {
        let mut b = minimal_event();
        b.ingest(prop(b"DTEND", b":19970903T190000Z")).unwrap();
        b.ingest(prop(b"DURATION", b":PT1H")).unwrap();
        assert!(matches!(
            b.build(true),
            Err(CalendarError::MutuallyExclusive("DTEND", "DURATION"))
        ));
    }

    #[test]
    fn event_builds_with_only_required_fields() {
        let event = minimal_event().build(true).unwrap();
        assert!(event.dtstart.is_some());
        assert!(event.alarms.is_empty());
    }

    fn minimal_todo() -> TodoBuilder {
        let mut b = TodoBuilder::new();
        b.ingest(prop(b"DTSTAMP", b":19970901T130000Z")).unwrap();
        b.ingest(prop(b"UID", b":123@example.com")).unwrap();
        b
    }

    #[test]
    fn todo_due_and_duration_are_mutually_exclusive() {
        let mut b = minimal_todo();
        b.ingest(prop(b"DTSTART", b":19970901T130000Z")).unwrap();
        b.ingest(prop(b"DUE", b":19970902T130000Z")).unwrap();
        b.ingest(prop(b"DURATION", b":PT1H")).unwrap();
        assert!(matches!(
            b.build(),
            Err(CalendarError::MutuallyExclusive("DUE", "DURATION"))
        ));
    }

    #[test]
    fn todo_duration_requires_dtstart() {
        let mut b = minimal_todo();
        b.ingest(prop(b"DURATION", b":PT1H")).unwrap();
        assert!(matches!(
            b.build(),
            Err(CalendarError::Requires("DURATION", "DTSTART"))
        ));
    }

    #[test]
    fn todo_builds_with_only_required_fields() {
        assert!(minimal_todo().build().is_ok());
    }

    fn minimal_alarm(action: &[u8]) -> AlarmBuilder {
        let mut b = AlarmBuilder::new();
        b.ingest(prop(b"ACTION", action)).unwrap();
        b.ingest(prop(b"TRIGGER", b":-PT15M")).unwrap();
        b
    }

    #[test]
    fn alarm_requires_action_and_trigger() {
        assert!(matches!(
            AlarmBuilder::new().build(),
            Err(CalendarError::MissingField("ACTION"))
        ));

        let mut with_action = AlarmBuilder::new();
        with_action.ingest(prop(b"ACTION", b":AUDIO")).unwrap();
        assert!(matches!(
            with_action.build(),
            Err(CalendarError::MissingField("TRIGGER"))
        ));
    }

    #[test]
    fn alarm_duration_and_repeat_must_occur_together() {
        let mut b = minimal_alarm(b":AUDIO");
        b.ingest(prop(b"DURATION", b":PT15M")).unwrap();
        assert!(matches!(
            b.build(),
            Err(CalendarError::RequiresTogether("DURATION", "REPEAT"))
        ));
    }

    #[test]
    fn alarm_audio_rejects_description() {
        let mut b = minimal_alarm(b":AUDIO");
        b.ingest(prop(b"DESCRIPTION", b":Beep")).unwrap();
        assert!(matches!(
            b.build(),
            Err(CalendarError::NotAllowed("DESCRIPTION", _))
        ));
    }

    #[test]
    fn alarm_display_requires_description() {
        let b = minimal_alarm(b":DISPLAY");
        assert!(matches!(
            b.build(),
            Err(CalendarError::MissingField("DESCRIPTION"))
        ));
    }

    #[test]
    fn alarm_email_requires_description_summary_and_attendee() {
        assert!(matches!(
            minimal_alarm(b":EMAIL").build(),
            Err(CalendarError::MissingField("DESCRIPTION"))
        ));

        let mut with_description = minimal_alarm(b":EMAIL");
        with_description
            .ingest(prop(b"DESCRIPTION", b":Reminder"))
            .unwrap();
        assert!(matches!(
            with_description.build(),
            Err(CalendarError::MissingField("SUMMARY"))
        ));

        let mut with_summary = minimal_alarm(b":EMAIL");
        with_summary
            .ingest(prop(b"DESCRIPTION", b":Reminder"))
            .unwrap();
        with_summary.ingest(prop(b"SUMMARY", b":Reminder")).unwrap();
        assert!(matches!(
            with_summary.build(),
            Err(CalendarError::RequiresAtLeastOne("ACTION EMAIL", "ATTENDEE"))
        ));

        let mut complete = minimal_alarm(b":EMAIL");
        complete.ingest(prop(b"DESCRIPTION", b":Reminder")).unwrap();
        complete.ingest(prop(b"SUMMARY", b":Reminder")).unwrap();
        complete
            .ingest(prop(b"ATTENDEE", b":mailto:jane@example.com"))
            .unwrap();
        assert!(complete.build().is_ok());
    }

    #[test]
    fn alarm_audio_builds_with_only_required_fields() {
        assert!(minimal_alarm(b":AUDIO").build().is_ok());
    }

    fn minimal_tz_prop() -> TzPropBuilder {
        let mut b = TzPropBuilder::new();
        b.ingest(prop(b"DTSTART", b":19710101T020000")).unwrap();
        b.ingest(prop(b"TZOFFSETFROM", b":-0400")).unwrap();
        b.ingest(prop(b"TZOFFSETTO", b":-0500")).unwrap();
        b
    }

    #[test]
    fn timezone_requires_at_least_one_observance() {
        let mut b = TimezoneBuilder::new();
        b.ingest(prop(b"TZID", b":America/New_York")).unwrap();
        assert!(matches!(
            b.build(),
            Err(CalendarError::RequiresAtLeastOne(
                "VTIMEZONE",
                "STANDARD or DAYLIGHT"
            ))
        ));
    }

    #[test]
    fn timezone_builds_with_one_standard_observance() {
        let mut b = TimezoneBuilder::new();
        b.ingest(prop(b"TZID", b":America/New_York")).unwrap();
        b.standardc.push(minimal_tz_prop());
        assert!(b.build().is_ok());
    }

    #[test]
    fn tz_prop_requires_dtstart_and_offsets() {
        assert!(matches!(
            TzPropBuilder::new().build(),
            Err(CalendarError::MissingField("DTSTART"))
        ));
    }

    #[test]
    fn calendar_requires_prodid_version_and_a_component() {
        assert!(matches!(
            CalendarBuilder::new().build(),
            Err(CalendarError::MissingField("PRODID"))
        ));

        let mut with_prodid = CalendarBuilder::new();
        with_prodid.prodid = Some(
            ProductIdentifier::try_from(b":-//example//EN".as_slice()).unwrap(),
        );
        assert!(matches!(
            with_prodid.build(),
            Err(CalendarError::MissingField("VERSION"))
        ));

        let mut with_version = CalendarBuilder::new();
        with_version.prodid = Some(
            ProductIdentifier::try_from(b":-//example//EN".as_slice()).unwrap(),
        );
        with_version.version =
            Some(Version::try_from(b":2.0".as_slice()).unwrap());
        assert!(matches!(
            with_version.build(),
            Err(CalendarError::RequiresAtLeastOne(
                "VCALENDAR",
                "calendar component"
            ))
        ));

        let mut complete = CalendarBuilder::new();
        complete.prodid = Some(
            ProductIdentifier::try_from(b":-//example//EN".as_slice()).unwrap(),
        );
        complete.version = Some(Version::try_from(b":2.0".as_slice()).unwrap());
        complete.components.push(minimal_event().into());
        assert!(complete.build().is_ok());
    }
}
