fn main() {
    use valuable::{NamedField, NamedValues, Value};

    let fields = [NamedField::new("foo"), NamedField::new("bar")];
    let values = [Value::U32(123), Value::U32(456)];

    let named_values = NamedValues::new(&fields, &values);

    let field = &fields[0];

    assert_eq!(named_values.get(field).and_then(Value::as_u32), Some(123));
}
