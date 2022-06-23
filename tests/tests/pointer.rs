use valuable::{pointer, Valuable, Value, Visit};

#[derive(Valuable)]
struct Struct1 {
    x: String,
    y: Struct2,
}

#[derive(Valuable)]
struct Struct2 {
    z: String,
}

#[derive(Default)]
struct CollectValues(Vec<String>);

impl Visit for CollectValues {
    fn visit_value(&mut self, value: Value<'_>) {
        self.0.push(format!("{:?}", value));
    }
}

#[test]
fn basic() {
    let value = Struct1 {
        x: "a".to_owned(),
        y: Struct2 { z: "b".to_owned() },
    };

    let mut visitor = CollectValues::default();
    value.visit_pointer(
        pointer::Pointer::new(&[pointer::Segment::Field("x")]),
        &mut visitor,
    );
    assert_eq!(visitor.0.len(), 1);
    assert_eq!(visitor.0[0], r#""a""#);

    let mut visitor = CollectValues::default();
    value.visit_pointer(
        pointer::Pointer::new(&[pointer::Segment::Field("y")]),
        &mut visitor,
    );
    assert_eq!(visitor.0.len(), 1);
    assert_eq!(visitor.0[0], r#"Struct2 { z: "b" }"#);

    let mut visitor = CollectValues::default();
    value.visit_pointer(
        pointer::Pointer::new(&[pointer::Segment::Field("y"), pointer::Segment::Field("z")]),
        &mut visitor,
    );
    assert_eq!(visitor.0.len(), 1);
    assert_eq!(visitor.0[0], r#""b""#);
}

#[cfg(feature = "derive")]
#[test]
fn visit_pointer_macro() {
    use valuable::visit_pointer;

    let value = Struct1 {
        x: "a".to_owned(),
        y: Struct2 { z: "b".to_owned() },
    };

    let mut visitor = CollectValues::default();
    visit_pointer!(value.x, visitor);
    assert_eq!(visitor.0.len(), 1);
    assert_eq!(visitor.0[0], r#""a""#);

    let mut visitor = CollectValues::default();
    visit_pointer!(value.y, visitor);
    assert_eq!(visitor.0.len(), 1);
    assert_eq!(visitor.0[0], r#"Struct2 { z: "b" }"#);

    let mut visitor = CollectValues::default();
    visit_pointer!(value.y.z, visitor);
    assert_eq!(visitor.0.len(), 1);
    assert_eq!(visitor.0[0], r#""b""#);
}
