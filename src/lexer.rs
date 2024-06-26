use crate::errors::{ErrorKind, JsonError};

use super::token::*;
use std::{iter::Peekable, str::Chars};

type Result<T> = std::result::Result<T, JsonError>;
/// The whitespace characters allowd in JSON according to the
/// The IETF JSON standard (RFC): https://datatracker.ietf.org/doc/html/rfc8259.
///
///
/// %x20 /              ; Space
/// %x09 /              ; Horizontal tab
/// %x0A /              ; Line feed or New line
/// %x0D )              ; Carriage return
const WHITESPACES: [char; 4] = ['\u{0020}', '\u{0009}', '\u{000A}', '\u{000D}'];

#[derive(Debug)]
pub(crate) struct Lexer<'a> {
    pub(crate) input_iter: Peekable<Chars<'a>>,
    pub(crate) line: usize,
}

impl<'a> Lexer<'a> {
    /// Initializes a new lexer with the given input.
    pub(crate) fn new(input: &'a str) -> Result<Self> {
        if input.is_empty() {
            return Err(JsonError::compose(ErrorKind::EmptyInput, None));
        }
        Ok(Self {
            input_iter: input.chars().peekable(),
            line: 1,
        })
    }

    /// Advances the iterator on the input.
    pub(crate) fn advance(&mut self) -> Option<char> {
        self.input_iter.next()
    }

    /// Peeks at the next character of the input
    pub(crate) fn peek(&mut self) -> Option<char> {
        self.input_iter.peek().copied()
    }

    /// Produces the next token.
    pub(crate) fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        match self.input_iter.peek() {
            Some('{') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Lbrace,
                })
            }
            Some('}') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Rbrace,
                })
            }
            Some(':') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Colon,
                })
            }
            Some(',') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Comma,
                })
            }
            Some('"') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Str,
                })
            }
            Some('[') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::LSqBracket,
                })
            }
            Some(']') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::RSqbracket,
                })
            }
            None => None,
            Some(ch) if !ch.is_ascii_digit() => Some(Token {
                token_type: TokenType::Character(*ch),
            }),
            Some(ch) if ch.is_ascii_digit() => Some(Token {
                token_type: TokenType::Digit,
            }),
            Some(_) => Some(Token {
                token_type: TokenType::Invalid,
            }),
        }
    }

    /// Consumes whitespace in the input and advances the iterator.
    pub(crate) fn skip_whitespace(&mut self) {
        while self.peek().filter(|ch| WHITESPACES.contains(ch)).is_some() {
            let current = self.advance();
            if current == Some('\n') {
                self.line += 1;
            }
        }
    }
}
