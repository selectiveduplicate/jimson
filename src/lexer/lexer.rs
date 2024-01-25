use crate::lexer::token::*;
use std::{clone, iter::Peekable, str::Chars};

type Result<T> = std::result::Result<T, LexerError>;

#[derive(Debug, Clone)]
pub enum LexerError {
    EmptyInput,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input_iter: Peekable<Chars<'a>>,
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
    fn advance(&mut self) {
        self.input_iter.next();
    }

    /// Produces the next token.
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        match self.input_iter.peek() {
            Some('{') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Lbrace,
                    token_literal: Some(String::from("{")),
                })
            }
            Some('}') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Rbrace,
                    token_literal: Some(String::from("}")),
                })
            }
            Some(':') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Colon,
                    token_literal: Some(String::from(":")),
                })
            }
            Some(',') => {
                self.advance();
                Some(Token {
                    token_type: TokenType::Comma,
                    token_literal: Some(String::from(",")),
                })
            }
            Some('"') => {
                self.advance();
                let string: String = self.parse_string();
                Some(Token {
                    token_type: TokenType::Str,
                    token_literal: Some(string),
                })
            }
            None => Some(Token {
                token_type: TokenType::Eof,
                token_literal: None,
            }),
            _ => Some(Token {
                token_type: TokenType::Invalid,
                token_literal: None,
            }),
        }
    }

    /// Parses a string key or value from the JSON.
    fn parse_string(&mut self) -> String {
        let mut string = String::new();
        while let Some(ch) = self.input_iter.peek() {
            if ch.eq(&'"') {
                self.advance();
                break;
            }
            string.push(*ch);
            self.advance();
        }
        string
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
