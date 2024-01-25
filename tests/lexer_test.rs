use jimson::lexer::lexer::Lexer;
use jimson::lexer::token::*;

#[test]
fn test_valid_json_with_empty_object() {
    let json = include_str!("inputs/step1/valid.json");
    let mut lexer = Lexer::new(json).unwrap();

    let expected = vec![
        Token {
            token_type: TokenType::Lbrace,
        },
        Token {
            token_type: TokenType::Rbrace,
        },
    ];

    for expected_token in expected {
        if let Some(result_token) = lexer.next_token() {
            assert_eq!(result_token, expected_token);
        }
    }
    assert!(!json.is_empty());
}

#[test]
fn test_invalid_json_file_empty() {
    let json = include_str!("inputs/step1/invalid.json");
    assert!(Lexer::new(json).is_err());
}

#[ignore = "not implemented"]
#[test]
fn test_valid_json_file_with_string_keys_and_values() {
    let json = include_str!("inputs/step2/valid.json");
    let mut lexer = Lexer::new(json).unwrap();

    let expected = vec![
        Token {
            token_type: TokenType::Lbrace,
        },
        Token {
            token_type: TokenType::Str,
        },
        Token {
            token_type: TokenType::Colon,
        },
        Token {
            token_type: TokenType::Str,
        },
        Token {
            token_type: TokenType::Rbrace,
        },
    ];
    for expected_token in expected {
        if let Some(result_token) = lexer.next_token() {
            assert_eq!(result_token, expected_token);
        }
    }
}

#[ignore = "not implemented"]
#[test]
fn test_valid_json_file_with_multiple_string_keys_and_values() {
    let json = include_str!("inputs/step2/valid2.json");
    let mut lexer = Lexer::new(json).unwrap();

    let expected = vec![
        Token {
            token_type: TokenType::Lbrace,
        },
        Token {
            token_type: TokenType::Str,
        },
        Token {
            token_type: TokenType::Colon,
        },
        Token {
            token_type: TokenType::Str,
        },
        Token {
            token_type: TokenType::Comma,
        },
        Token {
            token_type: TokenType::Str,
        },
        Token {
            token_type: TokenType::Colon,
        },
        Token {
            token_type: TokenType::Str,
        },
        Token {
            token_type: TokenType::Rbrace,
        },
        
    ];
    for expected_token in expected {
        if let Some(result_token) = lexer.next_token() {
            assert_eq!(result_token, expected_token);
        }
    }
}
