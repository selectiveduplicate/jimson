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

impl<'l> Parser<'l> {
    /// Create a new parser for the JSON data.
    pub fn new(input: &'l str) -> Result<Self, ParserError> {
        let lexer = match Lexer::new(input) {
            Ok(l) => l,
            Err(e) => return Err(ParserError::LexerError(e)),
        };
        Ok(Self { lexer })
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

    /// Parses a JSON object.
    fn parse_object(&mut self) -> Result<(), ParserError> {
        Ok(())
    }
}
