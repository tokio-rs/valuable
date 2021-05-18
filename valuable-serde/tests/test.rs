use std::collections::BTreeMap;

use serde::Serialize;
use serde_test::{assert_ser_tokens, Token};
use valuable::*;
use valuable_serde::Serializable;

macro_rules! assert_ser_eq {
    ($value:expr, &[$($tokens:tt)*] $(,)?) => {{
        let value = &$value;
        assert_ser_tokens(&Serializable::new(value), &[$($tokens)*]);
        assert_ser_tokens(value, &[$($tokens)*]);
    }};
}

#[test]
fn test_bool() {
    assert_ser_eq!(false, &[Token::Bool(false)]);
    assert_ser_eq!(true, &[Token::Bool(true)]);
}

#[test]
fn test_int() {
    assert_ser_eq!(i8::MIN, &[Token::I8(i8::MIN)]);
    assert_ser_eq!(i8::MAX, &[Token::I8(i8::MAX)]);
    assert_ser_eq!(i16::MIN, &[Token::I16(i16::MIN)]);
    assert_ser_eq!(i16::MAX, &[Token::I16(i16::MAX)]);
    assert_ser_eq!(i32::MIN, &[Token::I32(i32::MIN)]);
    assert_ser_eq!(i32::MAX, &[Token::I32(i32::MAX)]);
    assert_ser_eq!(i64::MIN, &[Token::I64(i64::MIN)]);
    assert_ser_eq!(i64::MAX, &[Token::I64(i64::MAX)]);
    // serde_test doesn't have Token::I128.
    // assert_ser_eq!(i128::MIN, &[Token::I128(i128::MIN)]);
    // assert_ser_eq!(i128::MAX, &[Token::I128(i128::MAX)]);
    assert_ser_eq!(isize::MIN, &[Token::I64(isize::MIN as _)]);
    assert_ser_eq!(isize::MAX, &[Token::I64(isize::MAX as _)]);
    assert_ser_eq!(u8::MAX, &[Token::U8(u8::MAX)]);
    assert_ser_eq!(u16::MAX, &[Token::U16(u16::MAX)]);
    assert_ser_eq!(u32::MAX, &[Token::U32(u32::MAX)]);
    assert_ser_eq!(u64::MAX, &[Token::U64(u64::MAX)]);
    // serde_test doesn't have Token::I128.
    // assert_ser_eq!(u128::MAX, &[Token::U128(u128::MAX)]);
    assert_ser_eq!(usize::MAX, &[Token::U64(usize::MAX as _)]);
}

#[test]
fn test_float() {
    assert_ser_eq!(f32::MIN, &[Token::F32(f32::MIN)]);
    assert_ser_eq!(f32::MAX, &[Token::F32(f32::MAX)]);
    assert_ser_eq!(f64::MIN, &[Token::F64(f64::MIN)]);
    assert_ser_eq!(f64::MAX, &[Token::F64(f64::MAX)]);
}

#[test]
fn test_char() {
    assert_ser_eq!('a', &[Token::Char('a')]);
}

#[test]
fn test_str() {
    assert_ser_eq!("a", &[Token::Str("a")]);
    assert_ser_eq!("a", &[Token::BorrowedStr("a")]);
    assert_ser_eq!("a", &[Token::String("a")]);
    assert_ser_eq!("a".to_string(), &[Token::Str("a")]);
    assert_ser_eq!("a".to_string(), &[Token::BorrowedStr("a")]);
    assert_ser_eq!("a".to_string(), &[Token::String("a")]);
}

// TODO
#[test]
fn test_option() {
    assert_ser_tokens(&Serializable::new(None::<u8>), &[Token::Unit]);
    assert_ser_tokens(&None::<u8>, &[Token::None]);
    assert_ser_tokens(&Serializable::new(Some(1)), &[Token::I32(1)]);
    assert_ser_tokens(&Some(1), &[Token::Some, Token::I32(1)]);
}

#[test]
fn test_unit() {
    assert_ser_eq!((), &[Token::Unit]);
}

// TODO
#[test]
fn test_unit_struct() {
    #[derive(Debug, PartialEq, Valuable, Serialize)]
    struct S;

    assert_ser_tokens(
        &Serializable::new(S),
        &[
            Token::TupleStruct { name: "S", len: 0 },
            Token::TupleStructEnd,
        ],
    );
    assert_ser_tokens(&S, &[Token::UnitStruct { name: "S" }]);
}

// TODO
#[test]
fn test_unit_variant() {
    #[derive(Debug, PartialEq, Valuable, Serialize)]
    enum E {
        V,
    }

    assert_ser_tokens(
        &Serializable::new(E::V),
        &[
            Token::TupleVariant {
                name: "E",
                variant: "V",
                len: 0,
            },
            Token::TupleVariantEnd,
        ],
    );
    assert_ser_tokens(
        &E::V,
        &[Token::UnitVariant {
            name: "E",
            variant: "V",
        }],
    );
}

#[test]
fn test_newtype_struct() {
    #[derive(Debug, PartialEq, Valuable, Serialize)]
    struct S(u8);

    assert_ser_eq!(S(0), &[Token::NewtypeStruct { name: "S" }, Token::U8(0)]);
}

#[test]
fn test_newtype_variant() {
    #[derive(Debug, PartialEq, Valuable, Serialize)]
    enum E {
        V(u8),
    }

    assert_ser_eq!(
        E::V(0),
        &[
            Token::NewtypeVariant {
                name: "E",
                variant: "V"
            },
            Token::U8(0)
        ]
    );
}

#[test]
fn test_seq() {
    assert_ser_eq!(
        vec![1, 2, 3],
        &[
            Token::Seq { len: Some(3) },
            Token::I32(1),
            Token::I32(2),
            Token::I32(3),
            Token::SeqEnd
        ]
    );
}

// TODO
// https://github.com/tokio-rs/valuable/issues/21
#[test]
fn test_tuple() {}

#[test]
fn test_tuple_struct() {
    #[derive(Debug, PartialEq, Valuable, Serialize)]
    struct S1();
    #[derive(Debug, PartialEq, Valuable, Serialize)]
    struct S2(i8, u8);

    assert_ser_eq!(
        S1(),
        &[
            Token::TupleStruct { name: "S1", len: 0 },
            Token::TupleStructEnd
        ]
    );
    assert_ser_eq!(
        S2(-1, 1),
        &[
            Token::TupleStruct { name: "S2", len: 2 },
            Token::I8(-1),
            Token::U8(1),
            Token::TupleStructEnd
        ]
    );
}

#[test]
fn test_tuple_variant() {
    #[derive(Debug, PartialEq, Valuable, Serialize)]
    enum E {
        V1(),
        V2(i8, u8),
    }

    assert_ser_eq!(
        E::V1(),
        &[
            Token::TupleVariant {
                name: "E",
                variant: "V1",
                len: 0
            },
            Token::TupleVariantEnd
        ]
    );
    assert_ser_eq!(
        E::V2(-1, 1),
        &[
            Token::TupleVariant {
                name: "E",
                variant: "V2",
                len: 2
            },
            Token::I8(-1),
            Token::U8(1),
            Token::TupleVariantEnd
        ]
    );
}

#[test]
fn test_map() {
    let mut m = BTreeMap::new();
    assert_ser_eq!(m, &[Token::Map { len: Some(0) }, Token::MapEnd]);
    m.insert(1, 10);
    m.insert(2, 20);
    assert_ser_eq!(
        m,
        &[
            Token::Map { len: Some(2) },
            Token::I32(1),
            Token::I32(10),
            Token::I32(2),
            Token::I32(20),
            Token::MapEnd
        ]
    );
}

#[test]
fn test_struct() {
    #[derive(Debug, PartialEq, Valuable, Serialize)]
    struct S1 {}
    #[derive(Debug, PartialEq, Valuable, Serialize)]
    struct S2 {
        f: u8,
    }

    assert_ser_eq!(
        S1 {},
        &[Token::Struct { name: "S1", len: 0 }, Token::StructEnd]
    );
    assert_ser_eq!(
        S2 { f: 1 },
        &[
            Token::Struct { name: "S2", len: 1 },
            Token::Str("f"),
            Token::U8(1),
            Token::StructEnd
        ]
    );
}

#[test]
fn test_struct_variant() {
    #[derive(Debug, PartialEq, Valuable, Serialize)]
    enum E {
        V1 {},
        V2 { f: u8 },
    }

    assert_ser_eq!(
        E::V1 {},
        &[
            Token::StructVariant {
                name: "E",
                variant: "V1",
                len: 0
            },
            Token::StructVariantEnd
        ]
    );
    assert_ser_eq!(
        E::V2 { f: 1 },
        &[
            Token::StructVariant {
                name: "E",
                variant: "V2",
                len: 1
            },
            Token::Str("f"),
            Token::U8(1),
            Token::StructVariantEnd
        ]
    );
}

#[test]
fn test_enum() {
    #[derive(Debug, PartialEq, Valuable, Serialize)]
    enum E {
        Unit,
        Newtype(u8),
        Tuple(u8, u8),
        Struct { f: u8 },
    }

    // TODO
    assert_ser_tokens(
        &Serializable::new(E::Unit),
        &[
            Token::Enum { name: "E" },
            Token::Str("Unit"),
            Token::Seq { len: Some(0) },
            Token::SeqEnd,
        ],
    );
    assert_ser_tokens(
        &E::Unit,
        &[Token::Enum { name: "E" }, Token::Str("Unit"), Token::Unit],
    );

    assert_ser_eq!(
        E::Newtype(0),
        &[
            Token::Enum { name: "E" },
            Token::Str("Newtype"),
            Token::U8(0),
        ],
    );

    assert_ser_eq!(
        E::Tuple(0, 0),
        &[
            Token::Enum { name: "E" },
            Token::Str("Tuple"),
            Token::Seq { len: Some(2) },
            Token::U8(0),
            Token::U8(0),
            Token::SeqEnd,
        ],
    );

    assert_ser_eq!(
        E::Struct { f: 0 },
        &[
            Token::Enum { name: "E" },
            Token::Str("Struct"),
            Token::Map { len: Some(1) },
            Token::Str("f"),
            Token::U8(0),
            Token::MapEnd,
        ],
    );
}

#[test]
fn test_dyn_struct() {
    struct Named;

    impl Valuable for Named {
        fn as_value(&self) -> Value<'_> {
            Value::Structable(self)
        }

        fn visit(&self, visit: &mut dyn Visit) {
            visit.visit_named_fields(&NamedValues::new(&[NamedField::new("a")], &[Value::U32(1)]));
            visit.visit_named_fields(&NamedValues::new(
                &[NamedField::new("b")],
                &[Value::I32(-1)],
            ));
        }
    }

    impl Structable for Named {
        fn definition(&self) -> StructDef<'_> {
            StructDef::new_dynamic("Named", Fields::Named(&[]))
        }
    }

    struct Unnamed;

    impl Valuable for Unnamed {
        fn as_value(&self) -> Value<'_> {
            Value::Structable(self)
        }

        fn visit(&self, visit: &mut dyn Visit) {
            visit.visit_unnamed_fields(&[Value::U32(1)]);
            visit.visit_unnamed_fields(&[Value::I32(-1)]);
        }
    }

    impl Structable for Unnamed {
        fn definition(&self) -> StructDef<'_> {
            StructDef::new_dynamic("Unnamed", Fields::Unnamed)
        }
    }

    assert_ser_tokens(
        &Serializable::new(Named),
        &[
            Token::Map { len: None },
            Token::Str("a"),
            Token::U32(1),
            Token::Str("b"),
            Token::I32(-1),
            Token::MapEnd,
        ],
    );

    assert_ser_tokens(
        &Serializable::new(Unnamed),
        &[
            Token::Seq { len: None },
            Token::U32(1),
            Token::I32(-1),
            Token::SeqEnd,
        ],
    );
}

#[test]
fn test_dyn_enum() {
    enum E {
        Named,
        Unnamed,
    }

    impl Valuable for E {
        fn as_value(&self) -> Value<'_> {
            Value::Enumerable(self)
        }

        fn visit(&self, visit: &mut dyn Visit) {
            match self {
                Self::Named => {
                    visit.visit_named_fields(&NamedValues::new(
                        &[NamedField::new("a")],
                        &[Value::U32(1)],
                    ));
                    visit.visit_named_fields(&NamedValues::new(
                        &[NamedField::new("b")],
                        &[Value::I32(-1)],
                    ));
                }
                Self::Unnamed => {
                    visit.visit_unnamed_fields(&[Value::U32(1)]);
                    visit.visit_unnamed_fields(&[Value::I32(-1)]);
                }
            }
        }
    }

    impl Enumerable for E {
        fn definition(&self) -> EnumDef<'_> {
            EnumDef::new_dynamic("E", &[])
        }

        fn variant(&self) -> Variant<'_> {
            match self {
                Self::Named => Variant::Dynamic(VariantDef::new("Named", Fields::Named(&[]))),
                Self::Unnamed => Variant::Dynamic(VariantDef::new("Named", Fields::Unnamed)),
            }
        }
    }

    assert_ser_tokens(
        &Serializable::new(E::Named),
        &[
            Token::Map { len: None },
            Token::Str("a"),
            Token::U32(1),
            Token::Str("b"),
            Token::I32(-1),
            Token::MapEnd,
        ],
    );

    assert_ser_tokens(
        &Serializable::new(E::Unnamed),
        &[
            Token::Seq { len: None },
            Token::U32(1),
            Token::I32(-1),
            Token::SeqEnd,
        ],
    );
}
