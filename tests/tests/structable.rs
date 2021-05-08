use valuable::field::*;
use valuable::*;

#[test]
fn test_named_field() {
    let name = "hello".to_string();
    let field = NamedField::new(&name[..]);
    assert_eq!(field.name(), "hello");

    let fields = [field];

    let fields = Fields::Named(&fields[..]);
    assert!(fields.is_named());
    assert!(!fields.is_unnamed());

    match fields {
        Fields::Named(..) => {}
        Fields::NamedStatic(..) | Fields::Unnamed => panic!(),
    }
}

#[test]
fn test_named_static_field() {
    static FIELDS: &[NamedField<'_>] = &[NamedField::new("hello")];

    let fields = Fields::NamedStatic(FIELDS);
    assert!(fields.is_named());
    assert!(!fields.is_unnamed());

    match fields {
        Fields::NamedStatic(..) => {}
        Fields::Named(..) | Fields::Unnamed => panic!(),
    }
}

#[test]
fn test_fields_unnamed() {
    let fields = Fields::Unnamed;
    assert!(fields.is_unnamed());
    assert!(!fields.is_named());
}

#[test]
fn test_struct_def() {
    let def = StructDef::new(
        "hello",
        Fields::Unnamed,
        false,
    );

    assert_eq!(def.name(), "hello");
    assert!(matches!(def.fields(), Fields::Unnamed));
    assert!(!def.is_dynamic());
}

#[test]
fn test_named_values() {
    let fields = [
        NamedField::new("foo"),
        NamedField::new("bar"),
    ];

    let vals = NamedValues::new(
        &fields[..],
        &[
            Value::U32(123),
            Value::String("hello"),
        ]
    );

    let other_field = NamedField::new("foo");

    assert!(matches!(vals.get(&fields[0]), Some(Value::U32(v)) if *v == 123));
    assert!(matches!(vals.get(&fields[1]), Some(Value::String(v)) if *v == "hello"));
    assert!(vals.get(&other_field).is_none());

    let e = vals.entries().collect::<Vec<_>>();
    assert_eq!(2, e.len());

    assert_eq!(e[0].0.name(), "foo");
    assert!(matches!(e[0].1, Value::U32(v) if *v == 123));

    assert_eq!(e[1].0.name(), "bar");
    assert!(matches!(e[1].1, Value::String(v) if *v == "hello"));
}
