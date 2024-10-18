use thiserror::Error;

pub(crate) type BoxError = Box<dyn std::error::Error + 'static>;
pub(crate) type Result<T> = std::result::Result<T, LineProtocolError>;

#[derive(Debug, Error)]
pub enum BuilderError {
    #[error("measurement name cannot be empty")]
    EmptyMeasurement,

    #[error("measurement name cannot start with '_' (underscore)")]
    InvalidMeasurement,

    #[error("tag key cannot be empty")]
    EmptyTagKey,

    #[error("tag key cannot start with '_' (underscore)")]
    InvalidTagKey,

    #[error("tag value cannot be empty")]
    EmptyTagValue,

    #[error("key cannot be empty")]
    EmptyFieldKey,

    #[error("key cannot start with '_' (underscore)")]
    InvalidFieldKey,

    #[error("value cannot be empty")]
    EmptyFieldValue,

    #[error("atleast one field is required")]
    MissingFields,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("line is a comment")]
    CommentLine,

    #[error("line cannot be empty")]
    EmptyLine,

    #[error("atleast one field is required")]
    MissingFields,

    #[error("timestamp is not a valid number")]
    InvalidTimestamp,

    #[error("invalid set: {0}")]
    InvalidSet(#[source] BoxError),
}

#[derive(Debug, Error)]
pub enum LineProtocolError {
    #[error("A builder error occured: {0}")]
    BuilderError(#[from] BuilderError),

    #[error("A parser error occured: {0}")]
    ParserError(#[from] ParseError),
}
