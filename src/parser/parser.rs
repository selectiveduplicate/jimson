use crate::lexer::lexer::*;
use crate::lexer::token::*;

/// Errors that can occur while parsing JSON data.
#[derive(Debug, Clone)]
pub enum ParserError {
    LexerError(LexerError),
    InvalidSyntax,
}

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
pub(crate) struct JsonValue<T> {
    value: T
}

/// A JSON object.
/// 
/// An object is a key-value pair, where key must be a string, and the value 
/// a `JsonValue` type.
pub(crate) struct JsonObject<T> {
    key: String,
    value: JsonValue<T>
}

impl<'l> Parser<'l> {
    /// Create a new parser for the JSON data.
    pub fn new(input: &'l str) -> Result<Self, ParserError> {
        let lexer = match Lexer::new(input) {
            Ok(l) => l,
            Err(e) => return Err(ParserError::LexerError(e)),
        };
        Ok(Self { lexer })
    }

    /// Parses a JSON object.
    fn parse_object(&mut self) -> Result<(), ParserError> {
        while let Some(tok) = self.lexer.next_token() {
            match tok.token_type {
                TokenType::Rbrace => break,
                _ => return Err(ParserError::InvalidSyntax)
            }
        }
        Ok(())
    }
    
    /// Parses the JSON data.
    pub fn parse(&mut self) -> Result<(), ParserError> {
        while let Some(tok) = self.lexer.next_token() {
            match tok.token_type {
                TokenType::Lbrace => self.parse_object()?,
                _ => return Err(ParserError::InvalidSyntax)
            }
        }
        Ok(())
    }

     /// Parses a string key or value from the JSON.
    fn parse_string(&mut self) -> Result<String, ParserError> {
        let mut string = String::new();
        while let Some(ch) = self.lexer.input_iter.peek() {
            if ch.eq(&'"') {
                self.lexer.advance();
                break;
            }
            string.push(*ch);
            self.lexer.advance();
        }
        Ok(string)
    }
}
