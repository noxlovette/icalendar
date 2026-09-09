pub trait DateTimeExt {
    fn from_ical(b: &[u8]) -> Self;
}
pub const ICAL_DATE_FMT: &str = "%Y%m%d"; // 20260909
pub const ICAL_DATETIME_FMT: &str = "%Y%m%dT%H%M%S"; // 20260909T153000  (floating/local)
pub const ICAL_DATETIME_UTC_FMT: &str = "%Y%m%dT%H%M%SZ"; // 20260909T153000Z (UTC)
