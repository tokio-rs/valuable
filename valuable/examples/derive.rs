#[cfg(feature = "valuable-derive",)]
use valuable::Valuable;

#[cfg(feature = "valuable-derive",)]
use std::collections::HashMap;

// `Debug` not implemented for struct, the debug implementation is going via
// valuable.
#[cfg(feature = "valuable-derive",)]
#[derive(Valuable)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
    favorites: HashMap<String, String>,
}

#[cfg(feature = "valuable-derive",)]
fn main() {
    let mut p = Person {
        name: "John Doe".to_string(),
        age: 42,
        phones: vec!["876-5309".to_string()],
        favorites: HashMap::new(),
    };

    p.favorites.insert("color".to_string(), "blue".to_string());

    println!("{:#?}", p.as_value());
}

#[cfg(not(feature = "valuable-derive",))]
fn main() {
}
