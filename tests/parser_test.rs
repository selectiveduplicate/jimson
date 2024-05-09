use jimson::parser::{JsonValue, Parser};

#[test]
fn create_a_new_parser_for_valid_json() {
    let json_parser = Parser::new(include_str!("inputs/step1/valid.json"));
    assert!(json_parser.is_ok());
}

#[test]
fn create_a_new_parser_for_empty_json() {
    let json_parser = Parser::new(include_str!("inputs/step1/invalid.json"));
    assert!(json_parser.is_err());
    assert_eq!(
        json_parser.unwrap_err().message,
        String::from("empty input")
    );
}

#[test]
fn parse_valid_json_containing_an_empty_object() {
    let mut json_parser = Parser::new(include_str!("inputs/step1/valid.json")).unwrap();
    let result = json_parser.parse();
    let JsonValue::Object(store) = result.unwrap() else {
        unreachable!()
    };
    assert!(store.is_empty());
}

#[test]
fn parse_valid_json_object_with_single_string_key_val_pair() {
    let mut json_parser = Parser::new(include_str!("inputs/step2/valid.json")).unwrap();
    let result = json_parser.parse();
    assert!(result.is_ok());
    let JsonValue::Object(store) = result.unwrap() else {
        unreachable!()
    };
    assert_eq!(store.len(), 1);
    let expected_value = JsonValue::String(String::from("value"));
    let expected_key = String::from("key");
    let (result_key, result_value) = store.get_key_value("key").unwrap();
    assert_eq!(&expected_value, result_value);
    assert_eq!(&expected_key, result_key);
}

#[test]
fn parse_invalid_json_object_with_trailing_comma() {
    let mut json_parser = Parser::new(include_str!("inputs/step2/invalid.json")).unwrap();
    let result = json_parser.parse();
    assert!(result.is_err());
}

#[test]
fn parse_valid_json_object_with_multiple_string_key_val_pairs() {
    let mut json_parser = Parser::new(include_str!("inputs/step2/valid2.json")).unwrap();
    let result = json_parser.parse();
    assert!(result.is_ok());

    let JsonValue::Object(store) = result.unwrap() else {
        unreachable!()
    };
    assert_eq!(store.len(), 2);

    let expected_value = JsonValue::String(String::from("value2"));
    let expected_key = String::from("key2");
    let (result_key, result_value) = store.get_key_value("key2").unwrap();
    assert_eq!(&expected_value, result_value);
    assert_eq!(&expected_key, result_key);
}

#[test]
fn parse_invalid_json_object_with_one_nonstring_key() {
    let mut json_parser = Parser::new(include_str!("inputs/step2/invalid2.json")).unwrap();
    let result = json_parser.parse().unwrap_err();
    assert_eq!(result.line, 3);
    assert_eq!(result.message, "object key must be string".to_string());
}

#[test]
fn parse_invalid_json_object_with_missing_brace_or_comma() {
    let mut json_parser = Parser::new(include_str!("inputs/step2/invalid3.json")).unwrap();
    let result = json_parser.parse().unwrap_err();
    assert_eq!(result.line, 2);
    assert_eq!(result.message, "expected curly brace or comma".to_string());
}

#[test]
fn parse_valid_json_object_with_null_value() {
    let mut json_parser = Parser::new(include_str!("inputs/step3/valid2.json")).unwrap();
    let result = json_parser.parse();
    assert!(result.is_ok());
}

#[test]
fn parse_valid_json_object_with_null_and_boolean_values() {
    let mut json_parser = Parser::new(include_str!("inputs/step3/valid.json")).unwrap();
    let result = json_parser.parse();
    assert!(result.is_ok());
}

#[test]
fn parse_invalid_json_object_with_invalid_boolean_value() {
    let mut json_parser = Parser::new(include_str!("inputs/step3/invalid2.json")).unwrap();
    let result = json_parser.parse().unwrap_err();
    assert_eq!(result.line, 3);
    assert_eq!(result.message, "invalid syntax".to_string());
}

#[test]
fn parse_valid_json_object_with_numbers() {
    let mut json_parser = Parser::new(include_str!("inputs/step3/valid3.json")).unwrap();
    let result = json_parser.parse();
    println!("{result:?}");
    assert!(result.is_ok());
}
