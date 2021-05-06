#![cfg(feature = "derive")]

use valuable::Valuable;

#[test]
fn test_derive_struct() {
    #[derive(Valuable)]
    struct Struct {
        x: &'static str,
    }

    #[derive(Valuable)]
    struct Tuple(u8);

    #[derive(Valuable)]
    struct Unit;

    let v = Struct { x: "a" };
    assert_eq!(format!("{:?}", v.as_value()), r#"Struct { x: "a" }"#);
    let v = Tuple(0);
    assert_eq!(format!("{:?}", v.as_value()), r#"Tuple(0)"#);
    let v = Unit;
    assert_eq!(format!("{:?}", v.as_value()), r#"Unit"#);
}

#[test]
fn test_derive_enum() {
    #[derive(Valuable)]
    enum Enum {
        Struct { x: &'static str },
        Tuple(u8),
        Unit,
    }

    let v = Enum::Struct { x: "a" };
    assert_eq!(format!("{:?}", v.as_value()), r#"Enum::Struct { x: "a" }"#);
    let v = Enum::Tuple(0);
    assert_eq!(format!("{:?}", v.as_value()), r#"Enum::Tuple(0)"#);
    let v = Enum::Unit;
    assert_eq!(format!("{:?}", v.as_value()), r#"Enum::Unit"#);
}
