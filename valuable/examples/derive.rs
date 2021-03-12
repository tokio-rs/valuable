use valuable::Valuable;

#[derive(Debug, Valuable)]
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

    println!("{:?}", p);
}