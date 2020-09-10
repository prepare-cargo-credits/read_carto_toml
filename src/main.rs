use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Cargo {
    package: Data,
    bin: Vec<Data>
}

fn main() {
    let text = std::fs::read_to_string("Cargo.toml").unwrap();
    let data: Cargo = toml::from_str(&text).unwrap();
    println!("{:?}", data);
}
