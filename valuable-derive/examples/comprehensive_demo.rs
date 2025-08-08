use valuable::Valuable;

/// Stack copy: bool -> bool
fn invert_flag(active: &bool) -> bool {
    !*active
}

/// Heap allocation: u64 -> String
fn format_id(id: &u64) -> String {
    format!("ID-{:06}", id)
}

/// Complex transformation: Vec<String> -> Box<[String]>
fn uppercase_list(items: &[String]) -> Box<[String]> {
    items
        .iter()
        .map(|s| s.to_uppercase())
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

#[derive(Valuable, Debug)]
struct Example {
    id: u64,

    #[valuable(with = "invert_flag")]
    active: bool,

    #[valuable(with = "format_id")]
    user_id: u64,

    #[valuable(with = "uppercase_list")]
    tags: Vec<String>,

    #[valuable(skip)]
    #[allow(dead_code)]
    secret: String,
}

fn main() {
    let example = Example {
        id: 42,
        active: false,
        user_id: 12345,
        tags: vec!["admin".to_string(), "user".to_string()],
        secret: "hidden".to_string(),
    };

    println!("Debug: {example:?}");

    struct TestVisitor {
        fields: Vec<(String, String)>,
    }

    impl valuable::Visit for TestVisitor {
        fn visit_value(&mut self, _value: valuable::Value<'_>) {}

        fn visit_named_fields(&mut self, named_values: &valuable::NamedValues<'_>) {
            for (field, value) in named_values {
                self.fields
                    .push((field.name().to_string(), format!("{value:?}")));
            }
        }

        fn visit_unnamed_fields(&mut self, values: &[valuable::Value<'_>]) {
            for (i, value) in values.iter().enumerate() {
                self.fields.push((i.to_string(), format!("{value:?}")));
            }
        }
    }

    let mut visitor = TestVisitor { fields: Vec::new() };
    example.visit(&mut visitor);

    println!("Valuable fields:");
    for (name, value) in visitor.fields {
        println!("  {}: {}", name, value);
    }
}
