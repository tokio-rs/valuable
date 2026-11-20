use valuable::*;

#[test]
fn test_iter() {
    let f = [NamedField::new("a"), NamedField::new("b")];
    let v = NamedValues::new(&f, &[Value::I32(1), Value::I32(2)]);

    let iter = v.iter();
    assert_eq!(iter.len(), 2);
    let v: Vec<_> = iter.map(|(f, v)| (f.name(), v.as_i32().unwrap())).collect();
    assert_eq!(v, vec![("a", 1), ("b", 2)]);
}

#[test]
fn test_iter_rev() {
    let f = [NamedField::new("a"), NamedField::new("b")];
    let v = NamedValues::new(&f, &[Value::I32(1), Value::I32(2)]);

    let iter = v.iter().rev();
    assert_eq!(iter.len(), 2);
    let v: Vec<_> = iter.map(|(f, v)| (f.name(), v.as_i32().unwrap())).collect();
    assert_eq!(v, vec![("b", 2), ("a", 1)]);
}

#[test]
fn test_get() {
    let f = [NamedField::new("a"), NamedField::new("b")];
    let bad = NamedField::new("a");
    let v = NamedValues::new(&f, &[Value::I32(1), Value::I32(2)]);

    assert!(matches!(v.get(&f[0]), Some(Value::I32(v)) if *v == 1));
    assert!(matches!(v.get(&f[1]), Some(Value::I32(v)) if *v == 2));
    assert!(v.get(&bad).is_none());
}

#[test]
fn test_get_empty() {
    let f = [NamedField::new("a")];
    let v = NamedValues::new(&[], &[]);

    assert!(v.get(&f[0]).is_none());
}

#[test]
fn test_get_field_before_fields() {
    let f = [NamedField::new("a"), NamedField::new("b")];
    let v = NamedValues::new(&f[1..], &[Value::I32(2)]);

    // `f[0]` lives below the start of the slice backing `v`.
    assert!(v.get(&f[0]).is_none());
    assert!(matches!(v.get(&f[1]), Some(Value::I32(v)) if *v == 2));
}

#[test]
fn test_get_by_name() {
    let f = [NamedField::new("a"), NamedField::new("b")];
    let v = NamedValues::new(&f, &[Value::I32(1), Value::I32(2)]);

    assert!(matches!(v.get_by_name("a"), Some(Value::I32(v)) if *v == 1));
    assert!(matches!(v.get_by_name("b"), Some(Value::I32(v)) if *v == 2));
    assert!(v.get_by_name("c").is_none());
}
