use super::errors::*;
use super::lexer::*;
use super::token::*;
use std::collections::HashMap;

/// A basic parser for JSON.
#[derive(Debug)]
pub struct Parser<'l> {
    lexer: Lexer<'l>,
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
#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    String(String),
    Boolean(bool),
    Number(f64),
    Null,
}

impl<'l> Parser<'l> {
    /// Numbers can be negative, and have decimal point.
    const NUMBER_CONSTRAINTS: [char; 2] = ['-', '.'];

    /// Returns error.
    pub(crate) fn error<T>(&mut self, msg: String) -> Result<T, JsonError> {
        return Err(JsonError {
            message: msg,
            line: self.lexer.line,
        });
    }

    /// Create a new parser for the JSON data.
    pub fn new(input: &'l str) -> Result<Self, JsonError> {
        let lexer = match Lexer::new(input) {
            Ok(lex) => lex,
            Err(e) => {
                if e == LexerError::EmptyInput {
                    return Err(JsonError {
                        line: 0,
                        message: String::from("empty input"),
                    });
                } else {
                    return Err(JsonError {
                        line: 0,
                        message: String::from("malformed input"),
                    });
                }
            }
        };
        Ok(Self { lexer })
    }

    /// Parses a JSON object.
    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        if let Some(ch) = self.lexer.peek() {
            if ch == '}' {
                return Ok(JsonValue::Object(HashMap::new()));
            }
        }
        let mut obj_store = HashMap::new();
        loop {
            let key = match self.parse() {
                Ok(JsonValue::String(s)) => s,
                Err(_) => return self.error(String::from("object key must be string")),
                Ok(_) => unreachable!(),
            };
            let Some(tok) = self.lexer.next_token() else {
                return self.error(String::from("end of input"));
            };
            if tok.token_type != TokenType::Colon {
                return self.error(String::from("expected colon"));
            }
            let value = self.parse()?;
            obj_store.insert(key, value);

            match self.lexer.next_token() {
                Some(tok) if tok.token_type == TokenType::Comma => {
                    self.lexer.skip_whitespace();
                    match self.lexer.peek() {
                        Some('"') => continue,
                        Some('}') => {
                            let msg = format!("trailing comma");
                            return Err(JsonError {
                                message: msg,
                                line: self.lexer.line - 1,
                            });
                        }
                        Some(ch) if ch.is_ascii_whitespace() => self.lexer.skip_whitespace(),
                        Some(ch) if ch.is_ascii_alphabetic() => continue,
                        _ => return self.error(String::from("invalid syntax")),
                    }
                }
                Some(tok) if tok.token_type == TokenType::Rbrace => break,
                Some(_) => return self.error(String::from("invalid syntax")),
                None => return self.error(String::from("expected curly brace or comma")),
            }
        }

        Ok(JsonValue::Object(obj_store))
    }

    /// Parses the JSON data.
    pub fn parse(&mut self) -> Result<JsonValue, JsonError> {
        let Some(tok) = self.lexer.next_token() else {
            return self.error(String::from("end of input"));
        };
        match tok.token_type {
            TokenType::Lbrace => self.parse_object(),
            TokenType::Str => self.parse_string(),
            TokenType::Character('n') => self.parse_null(),
            TokenType::Character('t') => self.parse_true(),
            TokenType::Character('f') => self.parse_false(),
            TokenType::Digit | TokenType::Character('-') => self.parse_number(),
            _ => return self.error(String::from("invalid syntax")),
        }
    }

    /// Parses an integer number value.
    fn parse_number(&mut self) -> Result<JsonValue, JsonError> {
        self.lexer.skip_whitespace();
        let mut string = String::new();
        while let Some(ch) = self.lexer.peek() {
            if !ch.is_ascii_digit() && !Self::NUMBER_CONSTRAINTS.contains(&ch) {
                //self.lexer.advance();
                break;
            }
            string.push(ch);
            self.lexer.advance();
        }
        let number = match string.parse() {
            Err(_) => return self.error(String::from("failed to parse number")),
            Ok(n) => n,
        };

        Ok(JsonValue::Number(number))
    }

    /// Helper function for `parse_null` and
    /// `parse_boolean`. Reads null and boolean values.
    fn read_keyword(&mut self, keyword: &'l str) -> Result<(), JsonError> {
        for c in keyword.chars() {
            let Some(current) = self.lexer.advance() else {
                // We're only doing this since we found the character 'n' in a
                // JSON value. Therefore, if the input stream ends before
                // we're done comparing with `keyword`,
                // that can only mean an invalid value.
                return self.error(String::from("invalid JSON value"));
            };
            if current != c {
                return self.error(String::from("invalid JSON value"));
            }
        }
        Ok(())
    }

    /// Parses the JSON `null` value.
    pub fn parse_null(&mut self) -> Result<JsonValue, JsonError> {
        self.read_keyword("null")?;
        Ok(JsonValue::Null)
    }

    /// Parses the `true` boolean value.
    pub fn parse_true(&mut self) -> Result<JsonValue, JsonError> {
        self.read_keyword("true")?;
        Ok(JsonValue::Boolean(true))
    }

    /// Parses the `false` boolean value.
    pub fn parse_false(&mut self) -> Result<JsonValue, JsonError> {
        self.read_keyword("false")?;
        Ok(JsonValue::Boolean(false))
    }

    /// Parses a string key or value from the JSON.
    fn parse_string(&mut self) -> Result<JsonValue, JsonError> {
        self.lexer.skip_whitespace();
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
