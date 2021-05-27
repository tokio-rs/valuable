use valuable::*;

macro_rules! test_tuple {
    (
        $(
            $name:ident => $num:expr, $tuple:expr;
        )*
    ) => {
        $(
            #[test]
            fn $name() {
                let tuple = $tuple;

                assert_eq!(format!("{:?}", tuple.as_value()), format!("{:?}", tuple),);

                let def = tuple.definition();

                assert_eq!(def.is_unit(), $num == 0);
                assert!(def.is_static());
                assert!(matches!(def, TupleDef::Static { fields, .. } if fields == $num));

                let counts = tests::visit_counts(&tuple);
                assert_eq!(
                    counts,
                    tests::VisitCount {
                        visit_unnamed_fields: 1,
                        ..Default::default()
                    }
                );
            }
        )*
    };
}

#[derive(Valuable, Debug)]
struct Foo {
    name: &'static str,
}

test_tuple! {
    test_0 => 0, ();
    test_1 => 1, (123,);
    test_2 => 2, (123, "foo");
    test_3 => 3, (123, "foo", "bar".to_string());
    test_4 => 4, ("bar".to_string(), 123, "foo", Foo { name: "Foo" });
    test_10 => 10, (
        1,
        "two",
        3_u64,
        4_f32,
        "five".to_string(),
        6,
        7,
        8,
        Foo { name: "nine" },
        10,
    );
}

#[test]
fn test_dyn_impl() {
    struct MyTuple;

    impl Valuable for MyTuple {
        fn as_value(&self) -> Value<'_> {
            Value::Tuplable(self)
        }

        fn visit(&self, visit: &mut dyn Visit) {
            visit.visit_unnamed_fields(&[Value::I32(123), Value::String("hello world")]);
            visit.visit_unnamed_fields(&[Value::String("j/k there is more")]);
        }
    }

    impl Tuplable for MyTuple {
        fn definition(&self) -> TupleDef {
            TupleDef::new_dynamic((0, None))
        }
    }

    let def = MyTuple.definition();
    assert!(!def.is_unit());
    assert!(def.is_dynamic());

    assert_eq!(
        format!("{:?}", MyTuple.as_value()),
        format!("{:?}", (123, "hello world", "j/k there is more")),
    );
}
