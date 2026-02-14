use jimson::parser::Parser;

fn main() {
    let json = r#"{
        "year": 2024,
        "rust": "awesome",
        "happiness": null,
        "sad": true,
        "temp": 39.4,
        "another": ['hat']
    }"#;

    let mut json_parser = Parser::new(json).expect("failed to initialize the parser");
    let parsed = match json_parser.parse() {
        Ok(d) => d,
        Err(e) => panic!("{e:?}"),
    };
    println!("{parsed:?}");
}
