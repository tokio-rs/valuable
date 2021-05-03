use valuable::Valuable;

// `Debug` not implemented for struct, the debug implementation is going via
// valuable.
#[derive(Valuable)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() {
    let p = Person {
        name: "John Doe".to_string(),
        age: 42,
        phones: vec!["876-5309".to_string()],
    };

    println!("{:#?}", p.as_value());
}
