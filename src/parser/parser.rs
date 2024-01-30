use std::any::Any;
use std::collections::HashMap;
use std::error;

use crate::lexer::lexer::*;
use crate::lexer::token::*;

/// Errors that can occur while parsing JSON data.
#[derive(Debug, Clone)]
pub enum ParserError {
    LexerError(LexerError),
    InvalidSyntax,
    MissingColon,
    ObjectKeyNotString,
    TrailingComma,
    MissingCurlyBraceOrComma
}

impl From<LexerError> for ParserError {
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
#[derive(Debug, PartialEq, Eq)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    String(String),
}

/// A JSON object.
///
/// An object is a key-value pair, where key must be a string, and the value
/// a `JsonValue` type.
#[derive(Debug)]
pub struct JsonObject {
    key: Option<String>,
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
        if let Some(ch) = self.lexer.peek() {
            if ch == '}' {
                return Ok(JsonValue::Object(HashMap::new()));
            }
        }
        let mut obj_store = HashMap::new();
        loop {
            let key = match self.parse() {
                Ok(JsonValue::String(s)) => s,
                Err(_) => return Err(ParserError::ObjectKeyNotString),
                Ok(_) => unreachable!(),
            };
            let Some(tok) = self.lexer.next_token() else {
                return Err(ParserError::LexerError(LexerError::EndOfInput));
            };
            if tok.token_type != TokenType::Colon {
                return Err(ParserError::MissingColon);
            }
            let value = self.parse()?;
            obj_store.insert(key, value);

            match self.lexer.next_token() {
                Some(tok) if tok.token_type == TokenType::Comma => {
                    self.lexer.skip_whitespace();
                    match self.lexer.peek() {
                        Some('"') => continue,
                        Some('}') => return Err(ParserError::TrailingComma),
                        Some(ch) if ch.is_ascii_whitespace() => self.lexer.skip_whitespace(),
                        Some(ch) if ch.is_ascii_alphabetic() => continue,
                        _ => return Err(ParserError::InvalidSyntax),
                    }
                }
                Some(tok) if tok.token_type == TokenType::Rbrace => break,
                Some(_) => return Err(ParserError::InvalidSyntax),
                None => return Err(ParserError::MissingCurlyBraceOrComma),
            }
        }

        Ok(JsonValue::Object(obj_store))
    }

    /// Parses the JSON data.
    pub fn parse(&mut self) -> Result<JsonValue, ParserError> {
        let Some(tok) = self.lexer.next_token() else {
            return Err(ParserError::LexerError(LexerError::EndOfInput));
        };
        match tok.token_type {
            TokenType::Lbrace => self.parse_object(),
            TokenType::Str => self.parse_string(),
            _ => Err(ParserError::InvalidSyntax),
        }
    }

    /// Parses a string key or value from the JSON.
    fn parse_string(&mut self) -> Result<JsonValue, ParserError> {
        self.lexer.skip_whitespace();
        //match self.lexer.peek() {
        //    Some(ch) => {
        //        if ch != '"' {
        //            return Err(ParserError::StringMissingDoubleQuotes);
        //        }
        //    }
        //    None => return Err(ParserError::LexerError(LexerError::EndOfInput)),
        //}
        //self.lexer.advance();
        let mut string = String::new();
        while let Some(ch) = self.lexer.peek() {
            if ch == '"' {
                self.lexer.advance();
                break;
            }
            string.push(ch);
            self.lexer.advance();
        }
        Ok(JsonValue::String(string))
    }
}
