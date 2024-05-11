#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: Option<usize>,
}

//#[derive(Debug, Clone)]
//pub enum ErrorKind {
//    InvalidSyntax,
//    InvalidJsonValue,
//    MissingColon,
//    ObjectKeyNotString,
//    TrailingComma,
//    MissingCurlyBraceOrComma,
//    ParseNumberError(std::num::ParseFloatError),
//}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //let msg = match self.kind {
        //    ErrorKind::MissingCurlyBraceOrComma => format!("expected curly brace or comma at the end on line {}", self.line),
        //    ErrorKind::InvalidSyntax => format!("invalid syntax on line {}", self.line),
        //    ErrorKind::InvalidJsonValue => format!("invalid JSON value on line {}", self.line),
        //    ErrorKind::MissingColon => format!("line {}: expected colon", self.line),
        //    ErrorKind::ObjectKeyNotString => format!("line {}: object key must be string", self.line),
        //    ErrorKind::TrailingComma => format!("line {}: trailing comma", self.line),
        //    ErrorKind::ParseNumberError(_) => format!("line {}: failed to parse number", self.line),
        //};
        if let Some(line) = self.line {
            write!(f, "line {}: {}", line, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for JsonError {}
