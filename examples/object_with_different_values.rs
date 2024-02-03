use jimson::Parser;

fn main() {
    let json = r#"{
        "year": 2024,
        "rust": "awesome",
        "happiness": null,
        "sad": true,
        "temp": 21.4
    }"#;

    let mut json_parser = Parser::new(json).expect("failed to initialize the parser");
    let parsed = match json_parser.parse() {
        Ok(d) => d,
        Err(e) => panic!("failed to parse JSON: {e:?}"),
    };
    println!("{parsed:?}");
}
