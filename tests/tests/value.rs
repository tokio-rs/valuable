use valuable::*;

use core::{isize, usize};

macro_rules! assert_visit_call {
    ($v:expr) => {
        let counts = tests::visit_counts($v);
        assert_eq!(
            counts,
            tests::VisitCount {
                visit_value: 1,
                ..Default::default()
            }
        );
    };
}

macro_rules! assert_as {
    ($val:expr, $src:expr, $variant:ident, $eq:ident, [$( $as:ident => $v:ident, )*]) => {
        $(
            if $val.$as().is_some() {
                let a = stringify!($variant);
                let b = stringify!($v);
                // numeric types convert between each other

                if !NUMERICS.contains(&a) || !NUMERICS.contains(&b) {
                    assert_eq!(stringify!($variant), stringify!($v));
                }
            } else {
                assert_ne!(stringify!($variant), stringify!($v));
            }
        )*
    }
}

macro_rules! assert_value {
    (
        $ty:ty: $variant:ident, $as:ident, $eq:ident => $( $values:expr ),*
    ) => {{
        use Value::*;

        struct VisitValue<'a>($ty, std::marker::PhantomData<&'a ()>);

        impl<'a> Visit for VisitValue<'a> {
            fn visit_value(&mut self, val: Value<'_>) {
                assert!(matches!(val, $variant(v) if $eq(&v, &self.0)));
            }
        }


        for &src in &[ $( $values ),* ] {
            // Visit the raw value once
            assert_visit_call!(&src);
            let mut visit = VisitValue(src, std::marker::PhantomData);
            src.visit(&mut visit);

            let val = Value::from(src);

            // Visit the converted value
            assert_visit_call!(&val);
            let mut visit = VisitValue(src, std::marker::PhantomData);
            val.visit(&mut visit);

            // Test conversion
            assert!(matches!(val, $variant(v) if $eq(&v, &src)));

            // Test `as_value()`
            assert!(matches!(Valuable::as_value(&val), $variant(v) if $eq(&v, &src)));

            // Test clone()
            assert!(matches!(val.clone(), $variant(v) if $eq(&v, &src)));

            // Test self as_*() conversion
            assert!($eq(&val.$as().unwrap(), &src));

            // Test all `as_*()` conversion
            assert_as!(val, src, $variant, $eq, [
                as_bool => Bool,
                as_char => Char,
                as_f32 => F32,
                as_f64 => F64,
                as_i8 => I8,
                as_i16 => I16,
                as_i32 => I32,
                as_i64 => I64,
                as_i128 => I128,
                as_isize => Isize,
                as_str => String,
                as_u8 => U8,
                as_u16 => U16,
                as_u32 => U32,
                as_u64 => U64,
                as_u128 => U128,
                as_usize => Usize,
                as_error => Error,
                as_listable => Listable,
                as_mappable => Mappable,
                as_structable => Structable,
                as_enumerable => Enumerable,
            ]);
        }
    }};
}

macro_rules! ints {
    (
        $( $n:expr ),*
     ) => {{
        use core::convert::TryFrom;

        vec![
            $(
                <u8>::try_from($n).ok().map(Value::from),
                <u16>::try_from($n).ok().map(Value::from),
                <u32>::try_from($n).ok().map(Value::from),
                <u64>::try_from($n).ok().map(Value::from),
                <u128>::try_from($n).ok().map(Value::from),
                <usize>::try_from($n).ok().map(Value::from),
                <i8>::try_from($n).ok().map(Value::from),
                <i16>::try_from($n).ok().map(Value::from),
                <i32>::try_from($n).ok().map(Value::from),
                <i64>::try_from($n).ok().map(Value::from),
                <i128>::try_from($n).ok().map(Value::from),
                <isize>::try_from($n).ok().map(Value::from),
            )*
        ]
        .into_iter()
        .filter_map(core::convert::identity)
        .collect::<Vec<_>>()
    }}
}

macro_rules! test_num {
    (
        $(
            $name:ident($as:ident, $ty:ty, $variant:ident);
        )*
     ) => {
         // Stringify all variants
         const NUMERICS: &[&str] = &[
            $(
                stringify!($variant),
            )*
         ];

        $(
            #[test]
            fn $name() {
                use core::convert::TryFrom;

                let mut valid = vec![];
                let mut invalid = vec![
                    Value::from(true),
                    Value::from('h'),
                    Value::from(3.14_f32),
                    Value::from(3.1415_f64),
                    Value::from("hello world"),
                ];

                for &shift in &[
                    0, 8, 16, 24, 32, 48, 64, 72, 80, 88, 96, 104, 112, 120, 126, 127
                ] {
                    let actual = u128::MAX.checked_shr(shift).unwrap();

                    match <$ty>::try_from(actual) {
                        Ok(v) => valid.push(v),
                        Err(_) => invalid.push(Value::from(actual)),
                    }
                }

                for &n in &valid {
                    assert_value!($ty: $variant, $as, eq => n);

                    for val in ints!(n) {
                        assert_eq!(Some(n), val.$as());
                    }
                }

                for val in &invalid {
                    assert!(val.$as().is_none());
                }
            }
        )*
    }
}

#[test]
fn test_default() {
    assert!(matches!(Value::default(), Value::Unit));
}

#[test]
fn test_bool() {
    assert_value!(bool: Bool, as_bool, eq => true, false);
}

#[test]
fn test_char() {
    assert_value!(char: Char, as_char, eq => 'a', 'b', 'c');
}

#[test]
fn test_f32() {
    assert_value!(f32: F32, as_f32, eq => 3.1415_f32, -1.234_f32, f32::MAX, f32::MIN);
}

#[test]
fn test_f64() {
    assert_value!(f64: F64, as_f64, eq => 3.1415_f64, -1.234_f64, f64::MAX, f64::MIN);
}

#[test]
fn test_str() {
    let string = "in a string".to_string();
    assert_value!(&'a str: String, as_str, eq => "hello world", &string);
}

#[test]
fn test_error() {
    use std::{error, io};

    let error: io::Error = io::ErrorKind::Other.into();
    let error: &dyn error::Error = &error;
    assert_value!(&'a dyn error::Error: Error, as_error, yes => error);
}

#[test]
fn test_unit() {
    use Value::Unit;

    struct VisitValue;

    impl Visit for VisitValue {
        fn visit_value(&mut self, val: Value<'_>) {
            assert!(matches!(val, Unit));
        }
    }

    // Visit the raw value once
    assert_visit_call!(&());
    let mut visit = VisitValue;
    ().visit(&mut visit);

    let val = Value::from(());

    // Visit the converted value
    assert_visit_call!(&());
    let mut visit = VisitValue;
    val.visit(&mut visit);

    // Test conversion
    assert!(matches!(val, Unit));

    // Test `as_value()`
    assert!(matches!(Valuable::as_value(&()), Unit));

    // Test clone()
    assert!(matches!(val.clone(), Unit));
}

test_num! {
    test_u8(as_u8, u8, U8);
    test_u16(as_u16, u16, U16);
    test_u32(as_u32, u32, U32);
    test_u64(as_u64, u64, U64);
    test_u128(as_u128, u128, U128);
    test_usize(as_usize, usize, Usize);
    test_i8(as_i8, i8, I8);
    test_i16(as_i16, i16, I16);
    test_i32(as_i32, i32, I32);
    test_i64(as_i64, i64, I64);
    test_i128(as_i128, i128, I128);
    test_isize(as_isize, isize, Isize);
}

fn eq<T: PartialEq>(a: &T, b: &T) -> bool {
    *a == *b
}

fn yes<T>(_: &T, _: &T) -> bool {
    true
}
