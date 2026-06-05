/// Alarm properties (Section 3.8.6): `ACTION`, `REPEAT`, `TRIGGER`.
pub mod alarms;
/// Change-management properties (Section 3.8.7): `CREATED`, `DTSTAMP`,
/// `LAST-MODIFIED`, `SEQUENCE`.
pub mod change;
/// Date and time properties (Section 3.8.2): `COMPLETED`, `DTEND`, `DUE`,
/// `DTSTART`, `DURATION`, `FREEBUSY`, `TRANSP`.
pub mod datetime;
/// Descriptive properties (Section 3.8.1): `ATTACH`, `CATEGORIES`, `CLASS`,
/// `COMMENT`, `DESCRIPTION`, `GEO`, `LOCATION`, `PERCENT-COMPLETE`, `PRIORITY`,
/// `RESOURCES`, `STATUS`, `SUMMARY`.
pub mod descriptive;
/// Miscellaneous properties (Section 3.8.8): `REQUEST-STATUS`.
pub mod misc;
/// Recurrence properties (Section 3.8.5): `EXDATE`, `RDATE`, `RRULE`.
pub mod recurrence;
/// Relationship properties (Section 3.8.4): `ATTENDEE`, `CONTACT`, `ORGANIZER`,
/// `RECURRENCE-ID`, `RELATED-TO`, `URL`, `UID`.
pub mod relationship;
/// Time zone properties (Section 3.8.3): `TZID`, `TZNAME`, `TZOFFSETFROM`,
/// `TZOFFSETTO`, `TZURL`.
pub mod timezone;
