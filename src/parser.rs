use super::errors::*;
use super::lexer::*;
use super::token::*;

use std::collections::HashMap;

const MAX_DEPTH: u8 = 15;

/// A basic parser for JSON.
#[derive(Debug)]
pub struct Parser<'l> {
    lexer: Lexer<'l>,
    depth: u8,
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
    Array(Vec<JsonValue>),
}

enum ContinueBreak {
    Continue,
    Break,
}

impl<'l> Parser<'l> {
    /// Numbers can be negative, and have decimal point.
    const NUMBER_CONSTRAINTS: [char; 2] = ['-', '.'];

    /// Create a new parser for the JSON data.
    pub fn new(input: &'l str) -> Result<Self, JsonError> {
        let lexer = Lexer::new(input)?;
        Ok(Self { lexer, depth: 0 })
    }

    /// Parses a JSON object.
    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        let mut obj_store = HashMap::new();

        if let Some('}') = self.lexer.peek() {
            self.lexer.advance();
            self.depth -= 1;
            return Ok(JsonValue::Object(obj_store));
        }
        loop {
            let key = match self.parse() {
                Ok(JsonValue::String(s)) => s,
                Err(_) => {
                    return Err(JsonError::compose(
                        ErrorKind::ObjectKeyNotString,
                        Some(self.lexer.line),
                    ))
                }
                _ => unreachable!(),
            };
            let Some(tok) = self.lexer.next_token() else {
                return Err(JsonError::compose(ErrorKind::Eof, Some(self.lexer.line)));
            };
            if tok.token_type != TokenType::Colon {
                return Err(JsonError::compose(
                    ErrorKind::MissingColon,
                    Some(self.lexer.line),
                ));
            }
            let value = self.parse()?;
            obj_store.insert(key, value);

            match self.check_obj()? {
                ContinueBreak::Continue => continue,
                ContinueBreak::Break => break,
            }
        }

        Ok(JsonValue::Object(obj_store))
    }

    fn check_arr(&mut self) -> Result<ContinueBreak, JsonError> {
        match self.lexer.next_token() {
            Some(tok) if tok.token_type == TokenType::Comma => {
                self.lexer.skip_whitespace();
                match self.lexer.peek() {
                    Some('"') => Ok(ContinueBreak::Continue),
                    Some(']') | Some(',') => Err(JsonError::compose(
                        ErrorKind::TrailingComma,
                        Some(self.lexer.line),
                    )),
                    _ => Ok(ContinueBreak::Continue),
                }
            }
            Some(tok) if tok.token_type == TokenType::RSqbracket => Ok(ContinueBreak::Break),
            Some(_) => Ok(ContinueBreak::Continue),
            None => Err(JsonError::compose(
                ErrorKind::UnclosedDelimiter(TokenType::RSqbracket),
                Some(self.lexer.line),
            )),
        }
    }

    fn check_obj(&mut self) -> Result<ContinueBreak, JsonError> {
        match self.lexer.next_token() {
            Some(tok) if tok.token_type == TokenType::Comma => {
                self.lexer.skip_whitespace();
                match self.lexer.peek() {
                    Some('"') => Ok(ContinueBreak::Continue),
                    Some('}') => Err(JsonError::compose(
                        ErrorKind::TrailingComma,
                        Some(self.lexer.line),
                    )),
                    _ => Ok(ContinueBreak::Continue),
                }
            }
            Some(tok) if tok.token_type == TokenType::Rbrace => Ok(ContinueBreak::Break),
            Some(_) => Err(JsonError::compose(
                ErrorKind::InvalidSyntax,
                Some(self.lexer.line),
            )),
            None => Err(JsonError::compose(
                ErrorKind::UnclosedDelimiter(TokenType::Rbrace),
                Some(self.lexer.line),
            )),
        }
    }

    /// Parses the JSON document.
    pub fn parse_root(&mut self) -> Result<JsonValue, JsonError> {
        // Any whitespace in the beginning.
        self.lexer.skip_whitespace();
        let value = self.parse()?;
        self.lexer.skip_whitespace();

        // Parsing the JSON document has finished so there should not be anything
        // left.
        if self.lexer.peek().is_some() {
            return Err(JsonError::compose(
                ErrorKind::InvalidSyntax,
                Some(self.lexer.line),
            ));
        }
        Ok(value)
    }

    pub(crate) fn parse(&mut self) -> Result<JsonValue, JsonError> {
        let Some(tok) = self.lexer.next_token() else {
            return Err(JsonError::compose(ErrorKind::Eof, Some(self.lexer.line)));
        };
        match tok.token_type {
            TokenType::Lbrace => {
                self.depth += 1;
                if self.depth > MAX_DEPTH {
                    return Err(JsonError::compose(
                        ErrorKind::NestingTooDeep,
                        Some(self.lexer.line),
                    ));
                }
                self.parse_object()
            }
            TokenType::Str => self.parse_string(),
            TokenType::Character('n') => self.parse_null(),
            TokenType::Character('t') => self.parse_true(),
            TokenType::Character('f') => self.parse_false(),
            TokenType::Digit | TokenType::Character('-') => self.parse_number(),
            TokenType::LSqBracket => {
                self.depth += 1;
                if self.depth > MAX_DEPTH {
                    return Err(JsonError::compose(
                        ErrorKind::NestingTooDeep,
                        Some(self.lexer.line),
                    ));
                }
                self.parse_array()
            }
            TokenType::InvalidChar('\'') => Err(JsonError::compose(
                ErrorKind::SingleQuote,
                Some(self.lexer.line),
            )),
            _ => Err(JsonError::compose(
                ErrorKind::InvalidSyntax,
                Some(self.lexer.line),
            )),
        }
    }

    /// Parses an array
    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        self.lexer.skip_whitespace();
        let mut array: Vec<JsonValue> = Vec::new();

        loop {
            if let Some(']') = self.lexer.peek() {
                self.lexer.advance();
                self.depth -= 1;
                return Ok(JsonValue::Array(array));
            }
            if let Some(',') = self.lexer.peek() {
                self.lexer.advance();
                return Err(JsonError::compose(
                    ErrorKind::MissingValue,
                    Some(self.lexer.line),
                ));
            }
            array.push(self.parse()?);
            match self.check_arr()? {
                ContinueBreak::Continue => continue,
                ContinueBreak::Break => break,
            }
        }
        Ok(JsonValue::Array(array))
    }

    /// Parses an integer number value.
    fn parse_number(&mut self) -> Result<JsonValue, JsonError> {
        self.lexer.skip_whitespace();
        if let Some('0') = self.lexer.peek() {
            return Err(JsonError::compose(
                ErrorKind::NumberWithLeadingZero,
                Some(self.lexer.line),
            ));
        }
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
            Err(e) => {
                return Err(JsonError::compose(
                    ErrorKind::ParseNumberError(e),
                    Some(self.lexer.line),
                ))
            }
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
                return Err(JsonError::compose(
                    ErrorKind::InvalidSyntax,
                    Some(self.lexer.line),
                ));
            };
            if current != c {
                return Err(JsonError::compose(
                    ErrorKind::InvalidSyntax,
                    Some(self.lexer.line),
                ));
            }
        }
        Ok(())
    }

    /// Parses the JSON `null` value.
    fn parse_null(&mut self) -> Result<JsonValue, JsonError> {
        self.read_keyword("null")?;
        Ok(JsonValue::Null)
    }

    /// Parses the `true` boolean value.
    fn parse_true(&mut self) -> Result<JsonValue, JsonError> {
        self.read_keyword("true")?;
        Ok(JsonValue::Boolean(true))
    }

    /// Parses the `false` boolean value.
    fn parse_false(&mut self) -> Result<JsonValue, JsonError> {
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
            if ch == '\\' {
                if let Some('\\') = self.lexer.peek() {
                    return Err(JsonError::compose(
                        ErrorKind::InvalidBackslashEscape,
                        Some(self.lexer.line),
                    ));
                }
            }
            string.push(ch);
            self.lexer.advance();
        }
        Ok(JsonValue::String(string))
    }
}
