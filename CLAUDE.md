# icalendar

A Rust iCalendar (RFC 5545) library.

## Style conventions

If you see a tuple struct like the Uid, NEVER EVER allow it to have its inner type to be PUBLIC

## Rule enforcement

Components MUST NOT USE ANYTHING OTHER THAN PROPERTIES

Properties MUST have a value and a params struct fields. The value field MUST contain only values from value.rs. Params must only contain values from params.rs, no exceptions

## Testing philosophy

Tests are written test-driven: they assert what the code's behavior MUST be per RFC 5545 (or the type's documented contract), not what the current implementation happens to do. Never write a test by running the code and copying its output into the assertion — that encodes bugs as spec. If a test fails against current code, the test is correct and the code is wrong; fix the code, don't loosen the test.

## Documentation conventions

Every public type and property struct/enum must have a doc comment that:

1. Includes the verbatim **Description** paragraph(s) from RFC 5545 (no "Description:" label).
2. Includes an **Example** section with a `>` blockquote showing the iCalendar wire format.
3. Ends with a RFC section link in the form:
   `[Section X.X.X](https://datatracker.ietf.org/doc/html/rfc5545#section-X.X.X)`

See `src/values.rs` and `src/properties/calendar.rs` for established examples of this pattern.

The authoritative reference is [RFC 5545](https://datatracker.ietf.org/doc/html/rfc5545).
