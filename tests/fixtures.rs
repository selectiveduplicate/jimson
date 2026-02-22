use std::{collections::HashMap, fs, path::Path};

use jimson::{
    errors::{ErrorKind, JsonError},
    parser::{JsonValue, Parser},
};

fn collect_test_files(path: &Path, tf: &mut Vec<std::path::PathBuf>) {
    for entry in
        fs::read_dir(path).unwrap_or_else(|e| panic!("failed to open directory {:?}: {}", path, e))
    {
        let file = entry.unwrap().path();
        if file.is_file() && file.extension().and_then(|e| e.to_str()) == Some("json") {
            tf.push(file);
        }
    }
    tf.sort();
}

#[test]
fn prepare_test_data_invalids() {
    let mut test_files = Vec::new();
    let p = Path::new("tests/fixtures/invalid");
    collect_test_files(p, &mut test_files);

    let expected: HashMap<&str, ErrorKind> = HashMap::from([
        (
            "fail1.json",
            ErrorKind::UnclosedDelimiter(jimson::token::TokenType::RSqbracket),
        ),
        ("fail2.json", ErrorKind::ObjectKeyNotString),
        ("fail3.json", ErrorKind::TrailingComma),
        ("fail4.json", ErrorKind::TrailingComma),
        ("fail5.json", ErrorKind::MissingValue),
        ("fail6.json", ErrorKind::InvalidSyntax),
        ("fail7.json", ErrorKind::InvalidSyntax),
        ("fail8.json", ErrorKind::TrailingComma),
        ("fail9.json", ErrorKind::InvalidSyntax),
        ("fail10.json", ErrorKind::InvalidSyntax),
        ("fail11.json", ErrorKind::InvalidSyntax),
        ("fail12.json", ErrorKind::NumberWithLeadingZero),
        ("fail13.json", ErrorKind::NumberWithLeadingZero),
        ("fail14.json", ErrorKind::InvalidBackslashEscape),
    ]);

    for path in test_files.iter() {
        let fname = path.file_name().unwrap().to_str().unwrap();
        let test_data = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed reading {:?}: {}", path, e));
        let parser = parse_test_json(test_data);
        assert!(&parser.unwrap_err().kind == expected.get(fname).unwrap());
    }
}

fn parse_test_json(j: String) -> Result<JsonValue, JsonError> {
    let mut parser = Parser::new(&j).unwrap();
    parser.parse_root()
}
