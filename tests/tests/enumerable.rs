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

        fn variant(&self) -> Variant<'_> {
            match *self {
                Enum::Struct { .. } => Variant::Static(&ENUM_VARIANTS[0]),
                Enum::Tuple(..) => Variant::Static(&ENUM_VARIANTS[1]),
                Enum::Unit => Variant::Static(&ENUM_VARIANTS[2]),
            }
        }
    }

    impl Valuable for Enum {
        fn as_value(&self) -> Value<'_> {
            Value::Enumerable(self)
        }

        fn visit(&self, visitor: &mut dyn Visit) {
            match self {
                Enum::Struct { x } => {
                    visitor.visit_named_fields(&NamedValues::new(
                        ENUM_STRUCT_FIELDS,
                        &[Value::String(x)],
                    ));
                }
                Enum::Tuple(y) => {
                    visitor.visit_unnamed_fields(&[Value::U8(*y)]);
                }
                Enum::Unit => {
                    visitor.visit_unnamed_fields(&[]);
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
fn test_variant_named_field() {
    let name = "my_field".to_string();
    let fields = [NamedField::new(&name[..])];
    let variant = VariantDef::new("Hello", Fields::Named(&fields[..]), false);

    assert_eq!(variant.name(), "Hello");
    assert!(!variant.is_dynamic());

    match *variant.fields() {
        Fields::Named(f) => {
            assert!(std::ptr::eq((&fields[..]).as_ptr(), f.as_ptr(),));
        }
        _ => panic!(),
    }
}

#[test]
fn test_variant_unnamed_field() {
    let variant = VariantDef::new("Hello", Fields::Unnamed, false);

    assert_eq!(variant.name(), "Hello");
    assert!(!variant.is_dynamic());
    assert!(matches!(variant.fields(), Fields::Unnamed));
}

#[test]
fn test_enum_def() {
    let fields = [NamedField::new("foo")];
    let a = VariantDef::new("A", Fields::Named(&fields[..]), false);
    let b = VariantDef::new("B", Fields::Unnamed, false);
    let variants = [a, b];
    let def = EnumDef::new("Foo", &variants, false);

    assert_eq!(def.name(), "Foo");
    assert!(std::ptr::eq(variants.as_ptr(), def.variants().as_ptr(),));
    assert!(!def.is_dynamic());
}
