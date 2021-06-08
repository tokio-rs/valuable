use std::{collections::BTreeMap, path::Path};

use valuable::*;
use valuable_json::*;

macro_rules! map {
    ($($key:expr => $val:expr),* $(,)?) => {{
        let mut _map = BTreeMap::new();
        $(
            _map.insert($key, $val);
        )*
        _map
    }};
}

#[track_caller]
fn assert_ser_compact<V>(value: V, expected: impl AsRef<str>)
where
    V: Valuable,
{
    let s = to_string(&value).unwrap();
    assert_eq!(s, expected.as_ref());
}

#[track_caller]
fn assert_ser_pretty<V>(value: V, expected: impl AsRef<str>)
where
    V: Valuable,
{
    let s = to_string_pretty(&value).unwrap();
    assert_eq!(s, expected.as_ref());
}

#[track_caller]
fn assert_ser_both<V>(value: V, expected: impl AsRef<str>)
where
    V: Valuable,
{
    assert_ser_compact(&value, expected.as_ref());
    assert_ser_pretty(&value, expected);
}

#[test]
fn test_bool() {
    assert_ser_both(false, "false");
    assert_ser_both(true, "true");
}

#[test]
fn test_bool_key() {
    let map = map!(
        true => 1,
        false => 0
    );
    assert_eq!(
        to_vec(&map).unwrap_err().to_string(),
        "bool cannot be a key"
    );
}

#[test]
fn test_int() {
    macro_rules! assert_int {
        ($($ty:ty),*) => {{
            $(
                assert_ser_both(<$ty>::MIN, <$ty>::MIN.to_string());
                assert_ser_both(<$ty>::MAX, <$ty>::MAX.to_string());
            )*
        }};
    }

    assert_int!(i8, i16, i32, i64, i128, isize);
    assert_int!(u8, u16, u32, u64, u128, usize);
}

#[test]
fn test_int_key() {
    macro_rules! assert_int_key {
        ($($ty:ty),*) => {{
            $(
                assert_ser_compact(
                    map!(
                        <$ty>::MAX => (),
                        <$ty>::MIN => (),
                    ),
                    to_string(&map!(
                        <$ty>::MAX.to_string() => (),
                        <$ty>::MIN.to_string() => (),
                    ))
                    .unwrap(),
                );
                assert_ser_pretty(
                    map!(
                        <$ty>::MAX => (),
                        <$ty>::MIN => (),
                    ),
                    to_string_pretty(&map!(
                        <$ty>::MAX.to_string() => (),
                        <$ty>::MIN.to_string() => (),
                    ))
                    .unwrap(),
                );
            )*
        }};
    }

    assert_int_key!(i8, i16, i32, i64, i128, isize);
    assert_int_key!(u8, u16, u32, u64, u128, usize);
}

#[test]
fn test_float() {
    assert_ser_both(f32::MIN, "-3.4028235e38");
    assert_ser_both(f32::MAX, "3.4028235e38");
    assert_ser_both(f32::EPSILON, "1.1920929e-7");
    assert_ser_both(f64::MIN, "-1.7976931348623157e308");
    assert_ser_both(f64::MAX, "1.7976931348623157e308");
    assert_ser_both(f64::EPSILON, "2.220446049250313e-16");
}

#[test]
fn test_float_key() {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct F32;
    impl Valuable for F32 {
        fn as_value(&self) -> Value<'_> {
            Value::F32(1.0)
        }

        fn visit(&self, visit: &mut dyn Visit) {
            visit.visit_value(self.as_value())
        }
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct F64;
    impl Valuable for F64 {
        fn as_value(&self) -> Value<'_> {
            Value::F64(1.0)
        }

        fn visit(&self, visit: &mut dyn Visit) {
            visit.visit_value(self.as_value())
        }
    }

    let map = map!(F32 => ());
    assert_eq!(
        to_string(&map).unwrap_err().to_string(),
        "float cannot be a key"
    );
    let map = map!(F64 => ());
    assert_eq!(
        to_string(&map).unwrap_err().to_string(),
        "float cannot be a key"
    );
}

#[test]
fn test_nonfinite_float_null() {
    assert_ser_both(f32::NAN, "null");
    assert_ser_both(f32::INFINITY, "null");
    assert_ser_both(f32::NEG_INFINITY, "null");
    assert_ser_both(f64::NAN, "null");
    assert_ser_both(f64::INFINITY, "null");
    assert_ser_both(f64::NEG_INFINITY, "null");
}

#[test]
fn test_nonfinite_float_error() {
    let mut out = vec![];
    assert_eq!(
        Serializer::new(&mut out)
            .ignore_nan(false)
            .serialize(&f32::NAN)
            .unwrap_err()
            .to_string(),
        "NaN cannot be a JSON value"
    );
    assert_eq!(
        Serializer::new(&mut out)
            .ignore_nan(false)
            .serialize(&f32::INFINITY)
            .unwrap_err()
            .to_string(),
        "infinity cannot be a JSON value"
    );
    assert_eq!(
        Serializer::new(&mut out)
            .ignore_nan(false)
            .serialize(&f32::NEG_INFINITY)
            .unwrap_err()
            .to_string(),
        "infinity cannot be a JSON value"
    );

    assert_eq!(
        Serializer::new(&mut out)
            .ignore_nan(false)
            .serialize(&f64::NAN)
            .unwrap_err()
            .to_string(),
        "NaN cannot be a JSON value"
    );
    assert_eq!(
        Serializer::new(&mut out)
            .ignore_nan(false)
            .serialize(&f64::INFINITY)
            .unwrap_err()
            .to_string(),
        "infinity cannot be a JSON value"
    );
    assert_eq!(
        Serializer::new(&mut out)
            .ignore_nan(false)
            .serialize(&f64::NEG_INFINITY)
            .unwrap_err()
            .to_string(),
        "infinity cannot be a JSON value"
    );
}

#[test]
fn test_char() {
    assert_ser_both('a', "\"a\"");
    assert_ser_both('"', "\"\\\"\"");
    assert_ser_both('\\', "\"\\\\\"");
    assert_ser_both('/', "\"/\"");
    assert_ser_both('\n', "\"\\n\"");
    assert_ser_both('\r', "\"\\r\"");
    assert_ser_both('\t', "\"\\t\"");
    assert_ser_both('\x08', "\"\\b\"");
    assert_ser_both('\x0C', "\"\\f\"");
    assert_ser_both('\x00', "\"\\u0000\"");
    assert_ser_both('\x1F', "\"\\u001f\"");
    assert_ser_both('\u{3A3}', "\"\u{3A3}\"");
}

#[test]
fn test_char_key() {
    assert_ser_compact(map!('a' => ()), to_string(&map!("a" => ())).unwrap());
    assert_ser_pretty(map!('a' => ()), to_string_pretty(&map!("a" => ())).unwrap());
}

#[test]
fn test_escape_solidus() {
    let mut out = vec![];
    Serializer::new(&mut out)
        .escape_solidus(true)
        .serialize(&"/")
        .unwrap();
    assert_eq!(String::from_utf8(out).unwrap(), "\"\\/\"");
}

#[test]
fn test_str() {
    assert_ser_both("", "\"\"");
    assert_ser_both("a", "\"a\"");
}

#[test]
fn test_path() {
    assert_ser_both(Path::new("a/b/c.txt"), "\"a/b/c.txt\"");
}

#[test]
fn test_path_key() {
    assert_ser_compact(
        map!(Path::new("a.txt") => ()),
        to_string(&map!("a.txt" => ())).unwrap(),
    );
    assert_ser_pretty(
        map!(Path::new("a.txt") => ()),
        to_string_pretty(&map!("a.txt" => ())).unwrap(),
    );
}

#[test]
fn test_option() {
    assert_ser_both(None::<u8>, "null");
    assert_ser_both(Some(1), "1");
    assert_ser_both(Some(()), "null");
    assert_ser_compact(Some(vec![0, 1]), "[0,1]");
    assert_ser_pretty(Some(vec![0, 1]), "[\n  0,\n  1\n]");
}

#[test]
fn test_unit() {
    assert_ser_both((), "null");
}

#[test]
fn test_unit_key() {
    let map = map!(
        () => (),
    );
    assert_eq!(
        to_vec(&map).unwrap_err().to_string(),
        "unit cannot be a key"
    );
}

#[test]
fn test_tuple() {
    assert_ser_compact((1,), r#"[1]"#);
    assert_ser_pretty((1,), "[\n  1\n]");
    assert_ser_compact(("a", 'b'), r#"["a","b"]"#);
    assert_ser_pretty(("a", 'b'), "[\n  \"a\",\n  \"b\"\n]");
}

#[test]
fn test_tuple_key() {
    let map = map!(
        ("a",) => (),
    );
    assert_eq!(
        to_vec(&map).unwrap_err().to_string(),
        "tuple cannot be a key"
    );
}

#[test]
fn test_list() {
    assert_ser_both(Vec::<u8>::new(), r#"[]"#);
    assert_ser_compact(vec![1, 2, 3], r#"[1,2,3]"#);
    assert_ser_pretty(vec![1, 2, 3], "[\n  1,\n  2,\n  3\n]");

    assert_ser_compact(vec![1, 2], r#"[1,2]"#);
    assert_ser_pretty(vec![1, 2], "[\n  1,\n  2\n]");
    assert_ser_compact(vec![vec![1, 2]], r#"[[1,2]]"#);
    assert_ser_pretty(vec![vec![1, 2]], "[\n  [\n    1,\n    2\n  ]\n]");
    assert_ser_compact(vec![Vec::<()>::new()], r#"[[]]"#);
    assert_ser_pretty(vec![Vec::<()>::new()], "[\n  []\n]");
}

#[test]
fn test_list_key() {
    let map = map!(
        vec!["a"] => (),
    );
    assert_eq!(
        to_vec(&map).unwrap_err().to_string(),
        "list cannot be a key"
    );
}

#[test]
fn test_unit_struct() {
    #[derive(Valuable)]
    struct S;

    assert_ser_both(S, "[]");
}

#[test]
fn test_tuple_struct() {
    #[derive(Valuable)]
    struct S0();
    #[derive(Valuable)]
    struct S1(u8);
    #[derive(Valuable)]
    struct S2(i8, u8);

    assert_ser_both(S0(), "[]");
    assert_ser_compact(S1(0), "[0]");
    assert_ser_pretty(S1(0), "[\n  0\n]");
    assert_ser_compact(S2(-1, 1), "[-1,1]");
    assert_ser_pretty(S2(-1, 1), "[\n  -1,\n  1\n]");
}

#[test]
fn test_map() {
    let mut m = BTreeMap::new();
    assert_ser_both(&m, r#"{}"#);
    m.insert("a", 10);
    assert_ser_compact(&m, r#"{"a":10}"#);
    assert_ser_pretty(&m, "{\n  \"a\": 10\n}");
    m.insert("b", 20);
    assert_ser_compact(&m, r#"{"a":10,"b":20}"#);
    assert_ser_pretty(&m, "{\n  \"a\": 10,\n  \"b\": 20\n}");
}

#[test]
fn test_map_key() {
    let map = map!(
        map!["a" => ()] => (),
    );
    assert_eq!(to_vec(&map).unwrap_err().to_string(), "map cannot be a key");
}

#[test]
fn test_struct() {
    #[derive(Valuable)]
    struct S0 {}
    #[derive(Valuable)]
    struct S1 {
        f: u8,
    }
    #[derive(Valuable)]
    struct S2 {
        f: u8,
        g: char,
    }

    assert_ser_both(S0 {}, r#"{}"#);
    assert_ser_compact(S1 { f: 1 }, r#"{"f":1}"#);
    assert_ser_pretty(S1 { f: 1 }, "{\n  \"f\": 1\n}");
    assert_ser_compact(S2 { f: 1, g: 'a' }, r#"{"f":1,"g":"a"}"#);
    assert_ser_pretty(S2 { f: 1, g: 'a' }, "{\n  \"f\": 1,\n  \"g\": \"a\"\n}");
}

#[test]
fn test_struct_key() {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Valuable)]
    struct Unit;
    #[derive(PartialEq, Eq, PartialOrd, Ord, Valuable)]
    struct Newtype(u8);
    #[derive(PartialEq, Eq, PartialOrd, Ord, Valuable)]
    struct Tuple(i8, u8);
    #[derive(PartialEq, Eq, PartialOrd, Ord, Valuable)]
    struct Struct {
        f: u8,
    }

    let map = map!(Unit => ());
    assert_eq!(
        to_vec(&map).unwrap_err().to_string(),
        "struct cannot be a key"
    );
    let map = map!(Newtype(0) => ());
    assert_eq!(
        to_vec(&map).unwrap_err().to_string(),
        "struct cannot be a key"
    );
    let map = map!(Tuple(-1, 1) => ());
    assert_eq!(
        to_vec(&map).unwrap_err().to_string(),
        "struct cannot be a key"
    );
    let map = map!(Struct { f: 0 } => ());
    assert_eq!(
        to_vec(&map).unwrap_err().to_string(),
        "struct cannot be a key"
    );
}

#[test]
fn test_enum() {
    #[derive(Valuable)]
    enum E {
        Unit,
        Newtype(u8),
        Tuple(i8, u8),
        Struct { f: u8 },
    }

    assert_ser_compact(E::Unit, r#"{"Unit":[]}"#);
    assert_ser_pretty(E::Unit, "{\n  \"Unit\": []\n}");
    assert_ser_compact(E::Newtype(0), r#"{"Newtype":[0]}"#);
    assert_ser_pretty(E::Newtype(0), "{\n  \"Newtype\": [\n    0\n  ]\n}");
    assert_ser_compact(E::Tuple(-1, 1), r#"{"Tuple":[-1,1]}"#);
    assert_ser_pretty(E::Tuple(-1, 1), "{\n  \"Tuple\": [\n    -1,\n    1\n  ]\n}");
    assert_ser_compact(E::Struct { f: 1 }, r#"{"Struct":{"f":1}}"#);
    assert_ser_pretty(
        E::Struct { f: 1 },
        "{\n  \"Struct\": {\n    \"f\": 1\n  }\n}",
    );
}

#[test]
fn test_enum_key() {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Valuable)]
    enum E {
        Unit,
        Newtype(u8),
        Tuple(i8, u8),
        Struct { f: u8 },
    }

    let map = map!(E::Unit => ());
    assert_eq!(
        to_vec(&map).unwrap_err().to_string(),
        "enum cannot be a key"
    );
    let map = map!(E::Newtype(0) => ());
    assert_eq!(
        to_vec(&map).unwrap_err().to_string(),
        "enum cannot be a key"
    );
    let map = map!(E::Tuple(-1, 1) => ());
    assert_eq!(
        to_vec(&map).unwrap_err().to_string(),
        "enum cannot be a key"
    );
    let map = map!(E::Struct { f: 0 } => ());
    assert_eq!(
        to_vec(&map).unwrap_err().to_string(),
        "enum cannot be a key"
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

    assert_ser_compact(Named, r#"{"a":1,"b":-1}"#);
    assert_ser_pretty(Named, "{\n  \"a\": 1,\n  \"b\": -1\n}");

    assert_ser_compact(Unnamed, "[1,-1]");
    assert_ser_pretty(Unnamed, "[\n  1,\n  -1\n]");
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
                Self::Unnamed => Variant::Dynamic(VariantDef::new("Unnamed", Fields::Unnamed)),
            }
        }
    }

    assert_ser_compact(E::Named, r#"{"Named":{"a":1,"b":-1}}"#);
    assert_ser_pretty(
        E::Named,
        "{\n  \"Named\": {\n    \"a\": 1,\n    \"b\": -1\n  }\n}",
    );
    assert_ser_compact(E::Unnamed, r#"{"Unnamed":[1,-1]}"#);
    assert_ser_pretty(E::Unnamed, "{\n  \"Unnamed\": [\n    1,\n    -1\n  ]\n}");
}
