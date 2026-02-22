use crate::token::TokenType;

#[derive(Debug, Clone)]
pub struct JsonError {
    pub kind: ErrorKind,
    pub line: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    InvalidSyntax,
    NotObjectOrArray,
    SingleQuote,
    MissingColon,
    ObjectKeyNotString,
    InvalidObjectValueType,
    TrailingComma,
    UnexpectedEof,
    UnclosedDelimiter(TokenType),
    MissingValue,
    EmptyInput,
    Eof,
    NumberWithLeadingZero,
    InvalidBackslashEscape,
    NestingTooDeep,
    ParseNumberError(std::num::ParseFloatError),
}

impl JsonError {
    /// Compose an error
    pub(crate) fn compose(kind: ErrorKind, line: Option<usize>) -> Self {
        JsonError { kind, line }
    }
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match &self.kind {
            ErrorKind::UnexpectedEof => "expected curly brace or comma".into(),
            ErrorKind::InvalidSyntax => "invalid syntax".into(),
            ErrorKind::MissingColon => "expected colon".into(),
            ErrorKind::ObjectKeyNotString => "object key must be string".into(),
            ErrorKind::TrailingComma => "trailing comma".into(),
            ErrorKind::Eof => "end of input".into(),
            ErrorKind::EmptyInput => "empty input".into(),
            ErrorKind::ParseNumberError(e) => format!("failed to parse number, {}", e),
            ErrorKind::SingleQuote => "single quote".into(),
            ErrorKind::NotObjectOrArray => "not an object or array".into(),
            ErrorKind::UnclosedDelimiter(_) => {
                "expected a closing delimiter of an object or array".into()
            }
            ErrorKind::MissingValue => "expected value".into(),
            ErrorKind::InvalidObjectValueType => "invalid object value type".into(),
            ErrorKind::NumberWithLeadingZero => "number cannot begin with zero".into(),
            ErrorKind::InvalidBackslashEscape => "invalid backslash escape".into(),
            ErrorKind::NestingTooDeep => "too many nested arrays or objects".into(),
        };
        if let Some(line) = self.line {
            write!(f, "error at line {}: {}", line, msg)
        } else {
            write!(f, "error: {}", msg)
        }
    }
}

impl std::error::Error for JsonError {}
