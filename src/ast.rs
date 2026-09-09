pub mod lexer;
pub mod parser;
mod token;
mod validator;
use parser::{ParseError, ParseResult};

use crate::{Calendar, properties::*};

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
    Timezone,
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
/// token's `literal()` carries) into the matching [`Property`] variant,
/// keyed by the token's upper-cased name (`lexeme()`).
type PropertyParser = fn(&[u8]) -> ParseResult<Property>;

/// Name -> parser dispatch table for every property RFC 5545 defines by
/// keyword (§3.7, §3.8). This is the single place that knows "PRODID means
/// a `ProductIdentifier`" — replacing what used to be a `TokenType` keyword
/// classified by the lexer itself. Built with [`phf`] (the same mechanism
/// the old lexer used for its `KEYWORDS` map) so the lookup stays O(1) —
/// a compile-time perfect hash, not a `match` over byte-string patterns
/// (which codegens as a comparison chain, not a jump table).
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

/// Dispatches a [`TokenType::Property`](super::ast::token::TokenType::Property)
/// token's name and raw remainder to the matching property type's
/// `TryFrom<&[u8]>`. A name absent from [`PROPERTY_DISPATCH`] isn't an
/// error — `X-`/IANA extension properties are open-ended by design
/// ([Section 3.8.8](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.8))
/// — it just falls back to [`Xprop`]/[`Iana`].
pub(crate) fn parse_property(
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
        todo!()
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum CalendarError {
    #[error("Missing field: {0}")]
    MissingField(&'static str),
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
    description: Option<Description>,
    geo: Option<Geo>,
    last_mod: Option<LastModified>,
    location: Option<Location>,
    organizer: Option<Organizer>,
    priority: Option<Priority>,
    seq: Option<Sequence>,
    status: Option<Status>,
    summary: Option<Summary>,
    transp: Option<String>,
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
    xprop: Option<Xprop>,
    iana: Option<Iana>,
}

impl EventBuilder {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
struct TodoBuilder {
    dtstamp: Option<DateTimeStamp>,
    uid: Option<Uid>,
    /// The following is REQUIRED if the component
    /// appears in an iCalendar object that doesn't
    /// specify the "METHOD" property; otherwise, it
    /// is OPTIONAL; in any case, it MUST NOT occur
    /// more than once.
    class: Option<Classification>,
    description: Option<Description>,
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
    xprop: Xprop,
    iana: Iana,
}

impl TodoBuilder {
    pub fn new() -> Self {
        todo!()
    }
}

impl FreeBusyBuilder {
    pub fn new() -> Self {
        todo!()
    }
}
impl JournalBuilder {
    pub fn new() -> Self {
        todo!()
    }
}
#[derive(Debug)]
struct FreeBusyBuilder {
    dtstamp: DateTimeStamp,
    uid: Uid,
    contact: Option<Contact>,
    dtstart: Option<DateTimeStart>,
    dtend: Option<DateTimeEnd>,
    organizer: Option<Organizer>,
    url: Option<UniformResourceLocator>,
    attendee: Vec<Attendee>,
    comment: Vec<Comment>,
    freebusy: Vec<FreeBusyTime>,
    rstatus: Vec<RequestStatus>,
    xprop: Xprop,
    iana: Iana,
}

#[derive(Debug)]
struct JournalBuilder {
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
