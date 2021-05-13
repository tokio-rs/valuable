use valuable::field::*;
use valuable::*;

#[test]
fn test_manual_static_impl() {
    struct MyStruct {
        num: u32,
        list: Vec<String>,
        sub: SubStruct,
    }

    static MY_STRUCT_FIELDS: &[NamedField<'static>] = &[
        NamedField::new("num"),
        NamedField::new("list"),
        NamedField::new("sub"),
    ];

    struct SubStruct {
        message: &'static str,
    }

    static SUB_STRUCT_FIELDS: &[NamedField<'static>] = &[NamedField::new("message")];

    impl Valuable for MyStruct {
        fn as_value(&self) -> Value<'_> {
            Value::Structable(self)
        }

        fn visit(&self, visit: &mut dyn Visit) {
            visit.visit_named_fields(&NamedValues::new(
                MY_STRUCT_FIELDS,
                &[
                    Value::U32(self.num),
                    Value::Listable(&self.list),
                    Value::Structable(&self.sub),
                ],
            ));
        }
    }

    impl Structable for MyStruct {
        fn definition(&self) -> StructDef<'_> {
            StructDef::new_static("MyStruct", Fields::Named(MY_STRUCT_FIELDS))
        }
    }

    impl Valuable for SubStruct {
        fn as_value(&self) -> Value<'_> {
            Value::Structable(self)
        }

        fn visit(&self, visit: &mut dyn Visit) {
            visit.visit_named_fields(&NamedValues::new(
                SUB_STRUCT_FIELDS,
                &[Value::String(self.message)],
            ));
        }
    }

    impl Structable for SubStruct {
        fn definition(&self) -> StructDef<'_> {
            StructDef::new_static("SubStruct", Fields::Named(SUB_STRUCT_FIELDS))
        }
    }

    let my_struct = MyStruct {
        num: 12,
        list: vec!["hello".to_string()],
        sub: SubStruct { message: "world" },
    };

    assert_eq!(
        format!("{:?}", my_struct.as_value()),
        "MyStruct { num: 12, list: [\"hello\"], sub: SubStruct { message: \"world\" } }"
    );
}

#[test]
fn test_manual_dyn_impl() {
    struct MyStruct;

    impl Valuable for MyStruct {
        fn as_value(&self) -> Value<'_> {
            Value::Structable(self)
        }

        fn visit(&self, visit: &mut dyn Visit) {
            visit.visit_named_fields(&NamedValues::new(
                &[NamedField::new("foo")],
                &[Value::U32(1)],
            ));
            visit.visit_named_fields(&NamedValues::new(
                &[NamedField::new("bar")],
                &[Value::String("two")],
            ));
        }
    }

    impl Structable for MyStruct {
        fn definition(&self) -> StructDef<'_> {
            StructDef::new_dynamic("MyStruct", Fields::Named(&[]))
        }
    }

    let my_struct = MyStruct;

    assert_eq!(
        format!("{:?}", my_struct.as_value()),
        "MyStruct { foo: 1, bar: \"two\" }"
    );
}

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
        _ => panic!(),
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
    let def = StructDef::new_static("hello", Fields::Unnamed);

    assert_eq!(def.name(), "hello");
    assert!(matches!(def.fields(), Fields::Unnamed));
    assert!(!def.is_dynamic());
}

#[test]
fn test_named_values() {
    let fields = [NamedField::new("foo"), NamedField::new("bar")];

    let vals = NamedValues::new(&fields[..], &[Value::U32(123), Value::String("hello")]);

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
