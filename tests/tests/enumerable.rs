use valuable::field::*;
use valuable::*;

#[test]
fn test_manual_static_impl() {
    enum Enum {
        Struct { x: &'static str },
        Tuple(u8),
        Unit,
    }

    static ENUM_STRUCT_FIELDS: &[NamedField<'static>] = &[NamedField::new("x")];
    static ENUM_VARIANTS: &[VariantDef<'static>] = &[
        VariantDef::new("Struct", Fields::NamedStatic(&ENUM_STRUCT_FIELDS), false),
        VariantDef::new("Tuple", Fields::Unnamed, false),
        VariantDef::new("Unit", Fields::Unnamed, false),
    ];

    impl Enumerable for Enum {
        fn definition(&self) -> EnumDef<'_> {
            EnumDef::new("Enum", ENUM_VARIANTS, false)
        }
    }

    impl Valuable for Enum {
        fn as_value(&self) -> Value<'_> {
            Value::Enumerable(self)
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

    let v = Enum::Struct { x: "a" };
    assert_eq!(format!("{:?}", v.as_value()), r#"Enum::Struct { x: "a" }"#);
    let v = Enum::Tuple(0);
    assert_eq!(format!("{:?}", v.as_value()), r#"Enum::Tuple(0)"#);
    let v = Enum::Unit;
    assert_eq!(format!("{:?}", v.as_value()), r#"Enum::Unit"#);
}

#[test]
#[ignore]
fn test_manual_dyn_impl() {
    todo!();
}

#[test]
#[ignore]
fn test_variant_named_field() {
    todo!()
}

#[test]
#[ignore]
fn test_variant_unnamed_field() {
    todo!()
}

#[test]
#[ignore]
fn test_variant_unnamed_fields() {
    todo!();
}

#[test]
#[ignore]
fn test_enum_def() {
    todo!();
}
