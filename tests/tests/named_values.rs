use valuable::field::*;
use valuable::*;

#[test]
fn test_entries() {
    let f = [NamedField::new("a"), NamedField::new("b")];
    let v = NamedValues::new(&f, &[Value::I32(1), Value::I32(2)]);

    let iter = v.entries();
    assert_eq!(iter.len(), 2);
    let v: Vec<_> = iter.map(|(f, v)| (f.name(), v.as_i32().unwrap())).collect();
    assert_eq!(v, vec![("a", 1), ("b", 2)]);
}

#[test]
fn test_entries_rev() {
    let f = [NamedField::new("a"), NamedField::new("b")];
    let v = NamedValues::new(&f, &[Value::I32(1), Value::I32(2)]);

    let iter = v.entries().rev();
    assert_eq!(iter.len(), 2);
    let v: Vec<_> = iter.map(|(f, v)| (f.name(), v.as_i32().unwrap())).collect();
    assert_eq!(v, vec![("b", 2), ("a", 1)]);
}
