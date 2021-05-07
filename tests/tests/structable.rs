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
