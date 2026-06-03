/// This property defines the status code returned for a scheduling request.
///
/// Example:
///
/// > REQUEST-STATUS:2.0;Success
///
/// [Section 3.8.8.3](https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.8.3)
pub struct RequestStatus(String);
