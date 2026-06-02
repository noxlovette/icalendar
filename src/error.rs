use crate::RRuleError;
use thiserror::Error;

/// Calendar parsing and recurrence processing errors.
#[derive(Debug, Error)]
pub enum CalendarError {
    /// Input value could not be parsed as a supported date format.
    #[error("Неподдерживаемый формат даты")]
    NotADate,
    /// Wrapped RRULE parsing error.
    #[error("Ошибка RRule")]
    RRule(#[from] RRuleError),
}

/// Errors that can occur while parsing an incoming VEVENT body.
#[derive(Debug, Error)]
pub enum VEventParseError {
    /// The body does not contain a VEVENT block.
    #[error("Отсутствует блок VEVENT")]
    NoVEvent,
    /// A required property is missing from the VEVENT.
    #[error("Отсутствует обязательное свойство: {0}")]
    MissingProperty(&'static str),
    /// A datetime value could not be parsed.
    #[error("Некорректный формат даты/времени: {0}")]
    InvalidDateTime(String),
    /// An unrecognised timezone identifier was encountered.
    #[error("Неизвестный часовой пояс: {0}")]
    UnknownTimezone(String),
}

/// Errors that can occur while parsing an incoming VTODO body.
#[derive(Debug, Error)]
pub enum VTodoParseError {
    /// The body does not contain a VTODO block.
    #[error("Отсутствует блок VTODO")]
    NoVTodo,
    /// A required property is missing from the VTODO.
    #[error("Отсутствует обязательное свойство: {0}")]
    MissingProperty(&'static str),
    /// A date value could not be parsed.
    #[error("Некорректный формат даты: {0}")]
    InvalidDate(String),
}
