use valuable::*;

#[test]
fn test_valuable_ref() {
    let val = &123;
    let val = Valuable::as_value(&val);
    assert!(matches!(val, Value::I32(v) if v == 123));
}

#[test]
fn test_valuable_box() {
    let val = Box::new(123);
    let val = Valuable::as_value(&val);
    assert!(matches!(val, Value::I32(v) if v == 123));
}
