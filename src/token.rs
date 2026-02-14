#[derive(Debug, Eq, PartialEq)]
/// The JSON tokens.
pub enum TokenType {
    Lbrace,
    Rbrace,
    LSqBracket,
    RSqbracket,
    Comma,
    Str,
    Colon,
    InvalidChar(char),
    Character(char),
    Digit,
}

#[derive(Debug, PartialEq, Eq)]
/// Represents a distinct token in JSON.
/// The token is identified by its `TokenType` and its string form.
pub struct Token {
    pub token_type: TokenType,
}
