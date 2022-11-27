#![cfg(feature = "derive")]

use valuable::*;

use std::collections::HashMap;
use std::env;

#[test]
fn test_derive_struct() {
    #[derive(Valuable)]
    struct Struct {
        x: &'static str,
    }

    #[derive(Valuable)]
    struct Tuple(u8);

    #[derive(Valuable)]
    struct Unit;

    let v = Struct { x: "a" };
    assert_eq!(format!("{:?}", v.as_value()), r#"Struct { x: "a" }"#);
    let v = Tuple(0);
    assert_eq!(format!("{:?}", v.as_value()), r#"Tuple(0)"#);
    let v = Unit;
    assert_eq!(format!("{:?}", v.as_value()), r#"Unit"#);
}

#[test]
fn test_derive_enum() {
    #[derive(Valuable)]
    enum Enum {
        Struct { x: &'static str },
        Tuple(u8),
        Unit,
    }

    let v = Enum::Struct { x: "a" };
    assert_eq!(format!("{:?}", v.as_value()), r#"Enum::Struct { x: "a" }"#);
    let v = Enum::Tuple(0);
    assert_eq!(format!("{:?}", v.as_value()), r#"Enum::Tuple(0)"#);
    let v = Enum::Unit;
    assert_eq!(format!("{:?}", v.as_value()), r#"Enum::Unit"#);
}

#[test]
fn test_derive_mut() {
    #[derive(Valuable)]
    struct S {
        _f: (),
    }

    #[derive(Valuable)]
    enum E {
        _V,
    }

    #[derive(Valuable)]
    struct Test<'a> {
        string: &'a mut String,
        list: &'a mut Vec<String>,
        map: &'a mut HashMap<String, String>,
        struct_: &'a mut S,
        enum_: &'a mut E,
    }
}

#[test]
fn test_rename() {
    #[derive(Valuable)]
    #[valuable(rename = "A")]
    struct S {
        #[valuable(rename = "b")]
        f: (),
    }

    #[derive(Valuable)]
    #[valuable(rename = "C")]
    enum E {
        #[valuable(rename = "D")]
        S {
            #[valuable(rename = "e")]
            f: (),
        },
        #[valuable(rename = "F")]
        T(()),
        #[valuable(rename = "G")]
        U,
    }

    let s = Structable::definition(&S { f: () });
    assert_eq!(s.name(), "A");
    assert!(matches!(s.fields(), Fields::Named(f) if f[0].name() == "b"));
    let e = Enumerable::definition(&E::S { f: () });
    assert_eq!(e.name(), "C");
    assert_eq!(e.variants()[0].name(), "D");
    assert!(matches!(e.variants()[0].fields(), Fields::Named(f) if f[0].name() == "e"));
    let e = Enumerable::definition(&E::T(()));
    assert_eq!(e.variants()[1].name(), "F");
    let e = Enumerable::definition(&E::U);
    assert_eq!(e.variants()[2].name(), "G");
}

#[test]
fn test_skip() {
    #[derive(Valuable)]
    struct S {
        #[allow(dead_code)]
        #[valuable(skip)]
        f: (),
    }

    #[derive(Valuable)]
    struct T(#[valuable(skip)] ());

    #[derive(Valuable)]
    enum E {
        S {
            #[valuable(skip)]
            f: (),
        },
        #[allow(dead_code)]
        T(#[valuable(skip)] ()),
    }

    let s = Structable::definition(&S { f: () });
    assert!(matches!(s.fields(), Fields::Named(f) if f.is_empty()));
    let _s = Structable::definition(&T(()));
    // assert!(matches!(s.fields() if f.is_empty()));
    let e = Enumerable::definition(&E::S { f: () });
    assert_eq!(e.variants().len(), 2);
    assert!(matches!(e.variants()[0].fields(), Fields::Named(f) if f.is_empty()));
    // assert!(matches!(e.variants()[1].fields() if f.is_empty()));
}

#[test]
fn test_transparent() {
    #[derive(Valuable)]
    #[valuable(transparent)]
    struct S {
        f: u8,
    }

    #[derive(Valuable)]
    #[valuable(transparent)]
    struct T(char);

    assert!(matches!(Valuable::as_value(&S { f: 0 }), Value::U8(0)));
    assert!(matches!(Valuable::as_value(&T('a')), Value::Char('a')));
}

#[rustversion::attr(not(stable), ignore)]
#[test]
fn ui() {
    // Do not require developers to manually set `TRYBUILD=overwrite`.
    if env::var_os("CI").is_none() {
        env::set_var("TRYBUILD", "overwrite");
    }

    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
