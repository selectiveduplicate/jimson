use std::any::Any;

use crate::lexer::lexer::*;
use crate::lexer::token::*;

/// Errors that can occur while parsing JSON data.
#[derive(Debug, Clone)]
pub enum ParserError {
    LexerError(LexerError),
    InvalidSyntax,
    ObjectKeyNotString,
    MissingColon,
}

impl  From<LexerError> for ParserError {
    fn from(value: LexerError) -> Self {
        ParserError::LexerError(value)
    }
}

/// A basic parser for JSON.
#[derive(Debug)]
pub struct Parser<'l> {
    lexer: Lexer<'l>,
}

/// JSON
#[derive(Debug)]
pub struct Json {
    element: Vec<JsonValue>,
}

/// A JSON value.
///
/// A value in JSON can be one of the following types:
/// 1. A string in double quotes
/// 2. A number
/// 3. A boolean
/// 4. A null
/// 5. An object
/// 6. An array
#[derive(Debug)]
pub enum JsonValue {
    Object(Box<JsonObject>),
    Str(String),
}

/// A JSON object.
///
/// An object is a key-value pair, where key must be a string, and the value
/// a `JsonValue` type.
#[derive(Debug)]
pub struct JsonObject {
    key: Option<JsonValue>,
    value: Option<JsonValue>,
}

impl<'l> Parser<'l> {
    /// Create a new parser for the JSON data.
    pub fn new(input: &'l str) -> Result<Self, ParserError> {
        let lexer = Lexer::new(input)?;
        Ok(Self { lexer })
    }

    /// Parses a JSON object.
    fn parse_object(&mut self) -> Result<JsonValue, ParserError> {
        self.lexer.skip_whitespace();
        if let Ok(ch) = self.lexer.peek() {
            if *ch == '}' {
                return Ok(JsonValue::Object(Box::new(JsonObject {
                    key: None,
                    value: None,
                })));
            }
        }
        let key = self.parse_string()?;
        let Some(tok) = self.lexer.next_token() else { return Err(ParserError::InvalidSyntax) ;};
        //let colon = self.lexer.next_token();
        if tok.token_type != TokenType::Colon {
            return Err(ParserError::MissingColon);
        } else {
            self.lexer.advance();
        }
        let value = self.parse_string()?;

        Ok(JsonValue::Object(Box::new(JsonObject {
            key: Some(key),
            value: Some(value),
        })))
    }

    /// Parses the JSON data.
    pub fn parse(&mut self) -> Result<JsonValue, ParserError> {
        let Some(tok) = self.lexer.next_token() else {
            todo!()
        };
        match tok.token_type {
            TokenType::Lbrace => self.parse_object(),
            _ => return Err(ParserError::InvalidSyntax),
        }
    }

    /// Parses a string key or value from the JSON.
    fn parse_string(&mut self) -> Result<JsonValue, ParserError> {
        match self.lexer.peek() {
            Ok(ch) => {
                if *ch != '"' {
                    return Err(ParserError::ObjectKeyNotString);
                }
            }
            Err(e) => return Err(ParserError::LexerError(e)),
        }
        self.lexer.advance();
        let mut string = String::new();
        while let Ok(ch) = self.lexer.peek() {
            if *ch == '"' {
                self.lexer.advance();
                break;
            }
            string.push(*ch);
            self.lexer.advance();
        }
        Ok(JsonValue::Str(string))
    }
}
