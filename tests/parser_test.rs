use jimson::{
    errors::ErrorKind,
    parser::{JsonValue, Parser},
    token::TokenType,
};

#[test]
fn create_a_new_parser_for_valid_json() {
    let json_parser = Parser::new(include_str!("inputs/step1/valid.json"));
    assert!(json_parser.is_ok());
}

#[test]
fn create_a_new_parser_for_empty_json() {
    let json_parser = Parser::new(include_str!("inputs/step1/invalid.json"));
    assert!(json_parser.is_err());
    assert_eq!(json_parser.unwrap_err().kind, ErrorKind::EmptyInput);
}

#[test]
fn parse_valid_json_containing_an_empty_object() {
    let mut json_parser = Parser::new(include_str!("inputs/step1/valid.json")).unwrap();
    let result = json_parser.parse_root();
    let JsonValue::Object(store) = result.unwrap() else {
        unreachable!()
    };
    assert!(store.is_empty());
}

#[test]
fn parse_valid_json_object_with_single_string_key_val_pair() {
    let mut json_parser = Parser::new(include_str!("inputs/step2/valid.json")).unwrap();
    let result = json_parser.parse_root();
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
    let result = json_parser.parse_root();
    assert!(result.is_err());
}

#[test]
fn parse_valid_json_object_with_multiple_string_key_val_pairs() {
    let mut json_parser = Parser::new(include_str!("inputs/step2/valid2.json")).unwrap();
    let result = json_parser.parse_root();
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
    let result = json_parser.parse_root().unwrap_err();
    assert_eq!(result.line.unwrap(), 3);
    assert_eq!(result.kind, ErrorKind::ObjectKeyNotString);
}

#[test]
fn parse_invalid_json_object_with_missing_brace_or_comma() {
    let mut json_parser = Parser::new(include_str!("inputs/step2/invalid3.json")).unwrap();
    let result = json_parser.parse_root().unwrap_err();
    assert_eq!(result.line.unwrap(), 2);
    assert_eq!(result.kind, ErrorKind::UnclosedDelimiter(TokenType::Rbrace));
}

#[test]
fn parse_valid_json_object_with_null_value() {
    let mut json_parser = Parser::new(include_str!("inputs/step3/valid2.json")).unwrap();
    let result = json_parser.parse_root();
    assert!(result.is_ok());
}

#[test]
fn parse_valid_json_object_with_null_and_boolean_values() {
    let mut json_parser = Parser::new(include_str!("inputs/step3/valid.json")).unwrap();
    let result = json_parser.parse_root();
    assert!(result.is_ok());
}

#[test]
fn parse_invalid_json_object_with_invalid_boolean_value() {
    let mut json_parser = Parser::new(include_str!("inputs/step3/invalid2.json")).unwrap();
    let result = json_parser.parse_root().unwrap_err();
    assert_eq!(result.line.unwrap(), 3);
    println!("{}", result);
    assert_eq!(result.kind, ErrorKind::InvalidSyntax);
}

#[test]
fn parse_valid_json_object_with_numbers() {
    let mut json_parser = Parser::new(include_str!("inputs/step3/valid3.json")).unwrap();
    let result = json_parser.parse_root();
    println!("{result:?}");
    assert!(result.is_ok());
}

#[test]
fn parse_valid_json_with_object_values() {
    let mut json_parser = Parser::new(include_str!("inputs/object_values.json")).unwrap();
    let result = json_parser.parse_root();
    assert!(result.is_ok());
}

#[test]
fn parse_valid_json_with_array_values() {
    let mut json_parser1 = Parser::new(include_str!("inputs/arrays.json")).unwrap();

    let mut json_parser2 = Parser::new(include_str!("inputs/step4/valid.json")).unwrap();
    let mut json_parser3 = Parser::new(include_str!("inputs/step4/valid2.json")).unwrap();
    let result1 = json_parser1.parse_root();
    let result2 = json_parser2.parse_root();
    let result3 = json_parser3.parse_root();

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());
}

#[test]
fn parse_invalid_json_with_array_values() {
    let mut json_parser = Parser::new(include_str!("inputs/step4/invalid.json")).unwrap();

    let result = json_parser.parse_root();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind, ErrorKind::SingleQuote);
}
