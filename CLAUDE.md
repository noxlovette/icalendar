# icalendar

A Rust iCalendar (RFC 5545) library.

## Style conventions

If you see a tuple struct like the Uid, NEVER EVER allow it to have its inner type to be PUBLIC

## Documentation conventions

Every public type and property struct/enum must have a doc comment that:

1. Includes the verbatim **Description** paragraph(s) from RFC 5545 (no "Description:" label).
2. Includes an **Example** section with a `>` blockquote showing the iCalendar wire format.
3. Ends with a RFC section link in the form:
   `[Section X.X.X](https://datatracker.ietf.org/doc/html/rfc5545#section-X.X.X)`

See `src/values.rs` and `src/properties/calendar.rs` for established examples of this pattern.

The authoritative reference is [RFC 5545](https://datatracker.ietf.org/doc/html/rfc5545).
