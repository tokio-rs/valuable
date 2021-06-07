use std::{collections::BTreeMap, path::Path};

use valuable::*;
use valuable_json::*;

#[track_caller]
fn assert_ser_eq<V>(value: V, expected: impl AsRef<str>)
where
    V: PartialEq + Valuable,
{
    let s = to_string(&value).unwrap();
    assert_eq!(s, expected.as_ref());
}

#[track_caller]
fn assert_ser_pretty_eq<V>(value: V, expected: impl AsRef<str>)
where
    V: PartialEq + Valuable,
{
    let s = to_string_pretty(&value).unwrap();
    assert_eq!(s, expected.as_ref());
}

#[track_caller]
fn assert_ser_both_eq<V>(value: V, expected: impl AsRef<str>)
where
    V: PartialEq + Valuable,
{
    assert_ser_eq(&value, expected.as_ref());
    assert_ser_pretty_eq(&value, expected);
}

#[test]
fn test_bool() {
    assert_ser_both_eq(false, "false");
    assert_ser_both_eq(true, "true");
}

#[test]
fn test_int() {
    assert_ser_both_eq(i8::MIN, i8::MIN.to_string());
    assert_ser_both_eq(i8::MAX, i8::MAX.to_string());
    assert_ser_both_eq(i16::MIN, i16::MIN.to_string());
    assert_ser_both_eq(i16::MAX, i16::MAX.to_string());
    assert_ser_both_eq(i32::MIN, i32::MIN.to_string());
    assert_ser_both_eq(i32::MAX, i32::MAX.to_string());
    assert_ser_both_eq(i64::MIN, i64::MIN.to_string());
    assert_ser_both_eq(i64::MAX, i64::MAX.to_string());
    assert_ser_both_eq(i128::MIN, i128::MIN.to_string());
    assert_ser_both_eq(i128::MAX, i128::MAX.to_string());
    assert_ser_both_eq(isize::MIN, isize::MIN.to_string());
    assert_ser_both_eq(isize::MAX, isize::MAX.to_string());
    assert_ser_both_eq(u8::MAX, u8::MAX.to_string());
    assert_ser_both_eq(u16::MAX, u16::MAX.to_string());
    assert_ser_both_eq(u32::MAX, u32::MAX.to_string());
    assert_ser_both_eq(u64::MAX, u64::MAX.to_string());
    assert_ser_both_eq(u128::MAX, u128::MAX.to_string());
    assert_ser_both_eq(usize::MAX, usize::MAX.to_string());
}

#[test]
fn test_float() {
    assert_ser_both_eq(f32::MIN, "-3.4028235e38");
    assert_ser_both_eq(f32::MAX, "3.4028235e38");
    assert_ser_both_eq(f32::EPSILON, "1.1920929e-7");
    assert_ser_both_eq(f64::MIN, "-1.7976931348623157e308");
    assert_ser_both_eq(f64::MAX, "1.7976931348623157e308");
    assert_ser_both_eq(f64::EPSILON, "2.220446049250313e-16");
}

#[test]
fn test_nonfinite_float_null() {
    assert_ser_both_eq(f32::NAN, "null");
    assert_ser_both_eq(f32::INFINITY, "null");
    assert_ser_both_eq(f32::NEG_INFINITY, "null");
    assert_ser_both_eq(f64::NAN, "null");
    assert_ser_both_eq(f64::INFINITY, "null");
    assert_ser_both_eq(f64::NEG_INFINITY, "null");
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
    assert_ser_both_eq('a', "\"a\"");
    // escape
    assert_ser_both_eq('"', "\"\\\"\"");
    assert_ser_both_eq('\\', "\"\\\\\"");
    assert_ser_both_eq('/', "\"/\"");
    assert_ser_both_eq('\n', "\"\\n\"");
    assert_ser_both_eq('\r', "\"\\r\"");
    assert_ser_both_eq('\t', "\"\\t\"");
    assert_ser_both_eq('\x08', "\"\\b\"");
    assert_ser_both_eq('\x0C', "\"\\f\"");
    assert_ser_both_eq('\x00', "\"\\u0000\"");
    assert_ser_both_eq('\x1F', "\"\\u001f\"");
    assert_ser_both_eq('\u{3A3}', "\"\u{3A3}\"");
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
    assert_ser_both_eq("", "\"\"");
    assert_ser_both_eq("a", "\"a\"");
}

#[test]
fn test_path() {
    assert_ser_both_eq(Path::new("a/b/c.txt"), "\"a/b/c.txt\"");
}

#[test]
fn test_option() {
    assert_ser_both_eq(None::<u8>, "null");
    assert_ser_both_eq(Some(1), "1");
    assert_ser_both_eq(Some(()), "null");
    assert_ser_eq(Some(vec![0, 1]), "[0,1]");
    assert_ser_pretty_eq(Some(vec![0, 1]), "[\n  0,\n  1\n]");
}

#[test]
fn test_unit() {
    assert_ser_both_eq((), "null");
}
