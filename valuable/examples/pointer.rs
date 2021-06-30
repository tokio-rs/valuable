// TODO: Change this example to doc example, and add tests.

use valuable::{visit_pointer, Pointer, PointerSegment, Valuable, Value, Visit};

#[derive(Valuable)]
struct Struct1 {
    field1: String,
    field2: Struct2,
}

#[derive(Valuable)]
struct Struct2 {
    field: String,
}

struct Visitor;

impl Visit for Visitor {
    fn visit_value(&mut self, value: Value<'_>) {
        println!("{:?}", value);
    }
}

fn main() {
    let value = Struct1 {
        field1: "a".to_owned(),
        field2: Struct2 {
            field: "b".to_owned(),
        },
    };

    let mut visitor = Visitor;

    value.visit_pointer(
        Pointer::new(&[PointerSegment::Field("field1")]),
        &mut visitor,
    );
    value.visit_pointer(
        Pointer::new(&[PointerSegment::Field("field2")]),
        &mut visitor,
    );
    value.visit_pointer(
        Pointer::new(&[
            PointerSegment::Field("field2"),
            PointerSegment::Field("field"),
        ]),
        &mut visitor,
    );

    visit_pointer!(value.field1, visitor); // output: "a"
    visit_pointer!(value.field2, visitor); // output: Struct2 { field: "b" }
    visit_pointer!(value.field2.field, visitor); // output: "b"
}
