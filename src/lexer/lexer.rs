use crate::lexer::token::*;
use std::{clone, iter::Peekable, str::Chars};

type Result<T> = std::result::Result<T, LexerError>;

#[derive(Debug, Clone)]
pub enum LexerError {
    EmptyInput,
    EndOfInput,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    pub(crate) input_iter: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    /// Initializes a new lexer with the given input.
    pub fn new(input: &'a str) -> Result<Self> {
        if input.is_empty() {
            return Err(LexerError::EmptyInput);
        }
        Ok(Self {
            input_iter: input.chars().peekable(),
        })
    }

    /// Advances the iterator on the input.
    pub(crate) fn advance(&mut self) {
        self.input_iter.next();
    }

    /// Peeks at the next character of the input
    pub(crate) fn peek(&mut self) -> std::result::Result<&char, LexerError> {
        self.input_iter.peek().ok_or(LexerError::EndOfInput)
    }

    /// Produces the next token.
    pub fn next_token(&mut self) -> Option<Token> {
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
                //let string: String = self.parse_string();
                Some(Token {
                    token_type: TokenType::Str,
                })
            }
            None => None,
            _ => Some(Token {
                token_type: TokenType::Invalid,
            }),
        }
    }

    /// Consumes whitespace in the input stream.
    fn skip_whitespace(&mut self) {
        while let Some(next_ch) = self.input_iter.peek() {
            if next_ch.is_ascii_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Parses a number as a string from the input.
    fn parse_number(&mut self) -> String {
        let mut number = String::new();
        while let Some(ch) = self.input_iter.peek() {
            if ch.is_ascii_digit() {
                number.push(*ch);
                self.input_iter.next();
            } else {
                break;
            }
        }
        number
    }
}
