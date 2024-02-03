use jimson::Parser;

fn main() {
    let json = r#"{
        "year": 2024,
        "rust": "awesome",
        "happiness": null,
        "temp": 21.4
    }"#;

    let mut json_parser = Parser::new(json).expect("failed to create parser for the JSON");
    let parsed = match json_parser.parse() {
        Ok(d) => d,
        Err(e) => panic!("failed to parse JSON: {e:?}"),
    };
    println!("{parsed:?}");
}
