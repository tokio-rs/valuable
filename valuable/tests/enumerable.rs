use valuable::field::*;
use valuable::*;

#[test]
fn test_manual_impl() {
    enum Enum {
        Struct { x: &'static str },
        Tuple(u8),
        Unit,
    }

    static ENUM_STRUCT_FIELDS: &[NamedField<'static>] = &[NamedField::new("x")];
    static ENUM_VARIANTS: &[VariantDef<'static>] = &[
        VariantDef {
            name: "Struct",
            fields: Fields::NamedStatic(&ENUM_STRUCT_FIELDS),
            is_dynamic: false,
        },
        VariantDef {
            name: "Tuple",
            fields: Fields::Unnamed,
            is_dynamic: false,
        },
        VariantDef {
            name: "Unit",
            fields: Fields::Unnamed,
            is_dynamic: false,
        },
    ];

    impl Enumerable for Enum {
        fn definition(&self) -> EnumDef<'_> {
            EnumDef {
                name: "Enum",
                variants: ENUM_VARIANTS,
                is_dynamic: false,
            }
        }

        fn visit(&self, visitor: &mut dyn Visit) {
            match self {
                Enum::Struct { x } => {
                    visitor.visit_variant_named_fields(
                        &Variant { name: "Struct" },
                        &NamedValues::new(ENUM_STRUCT_FIELDS, &[Value::String(x)]),
                    );
                }
                Enum::Tuple(y) => {
                    visitor
                        .visit_variant_unnamed_fields(&Variant { name: "Tuple" }, &[Value::U8(*y)]);
                }
                Enum::Unit => {
                    visitor.visit_variant_unnamed_fields(&Variant { name: "Unit" }, &[]);
                }
            }
        }
    }

    impl Valuable for Enum {
        fn as_value(&self) -> Value<'_> {
            Value::Enumerable(self)
        }
    }

    let v = Enum::Struct { x: "a" };
    assert_eq!(format!("{:?}", v.as_value()), r#"Enum::Struct { x: "a" }"#);
    let v = Enum::Tuple(0);
    assert_eq!(format!("{:?}", v.as_value()), r#"Enum::Tuple(0)"#);
    let v = Enum::Unit;
    assert_eq!(format!("{:?}", v.as_value()), r#"Enum::Unit"#);
}
