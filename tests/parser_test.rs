use jimson::lexer::lexer::*;
use jimson::parser::parser::{JsonObject, JsonValue, Parser, ParserError};

#[test]
fn create_a_new_parser_for_valid_json() {
    let json_parser = Parser::new(include_str!("inputs/step1/valid.json"));
    assert!(json_parser.is_ok());
}

#[test]
fn create_a_new_parser_for_empty_json() {
    let json_parser = Parser::new(include_str!("inputs/step1/invalid.json"));
    assert!(json_parser.is_err());
    assert!(matches!(json_parser, Err(ParserError::LexerError(_))));
}

#[test]
fn parse_valid_json_containing_an_empty_object() {
    let mut json_parser = Parser::new(include_str!("inputs/step1/valid.json")).unwrap();
    assert!(json_parser.parse().is_ok());
}

#[test]
fn parse_valid_json_object_with_single_string_key_val_pair() {
    let mut json_parser = Parser::new(include_str!("inputs/step2/valid.json")).unwrap();
    let result = json_parser.parse();
    println!("{:?}", result);
    assert!(result.is_ok());
}