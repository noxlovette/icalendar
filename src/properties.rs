/// Section 3,7
mod calendar;
/// Section 3,8
mod component;

use std::io::Write;

use bytes::Bytes;
pub use calendar::*;
pub use component::*;

use crate::ParseResult;

/// Properties
pub trait Property: Sized {
    /// Writes the property to buffer
    fn write(&self, w: impl Write) -> ParseResult<()>;

    /// Parses the property from bytes
    fn parse(b: Bytes) -> ParseResult<()>;
}
