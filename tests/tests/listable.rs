use tests::*;
use valuable::*;

struct VisitHello(u32);

impl Visit for VisitHello {
    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        let id = &HELLO_WORLD_FIELDS[0];
        assert_eq!("id", id.name());
        assert_eq!(Some(self.0), named_values.get(id).unwrap().as_u32());
    }

    fn visit_value(&mut self, _: Value<'_>) {
        unreachable!("not called in this test");
    }
}

#[derive(Default)]
struct VisitList(u32);

impl Visit for VisitList {
    fn visit_value(&mut self, item: Value<'_>) {
        visit_hello(&item, self.0);
        self.0 += 1;
    }
}

fn visit_hello(value: &Value<'_>, expect: u32) {
    let value = value.as_structable().unwrap();

    // Check only one visit method is called
    let counts = tests::visit_counts(&value);
    assert_eq!(
        counts,
        tests::VisitCount {
            visit_named_fields: 1,
            ..Default::default()
        }
    );

    // Check the next ID
    let mut v = VisitHello(expect);
    value.visit(&mut v);
}

macro_rules! test_default {
    (
        $(
            $name:ident => |$x:ident| $b:expr;
        )*
    ) => {
        $(
            mod $name {
                use super::*;
                #[test]
                fn test_default_visit_slice_empty() {
                    let $x: Vec<HelloWorld> = vec![];
                    let empty = $b;

                    assert_eq!(Listable::size_hint(&empty), (0, Some(0)));

                    let counts = tests::visit_counts(&empty);
                    assert_eq!(
                        counts,
                        Default::default(),
                    );

                    let mut counts = tests::VisitCount::default();
                    valuable::visit(&empty, &mut counts);
                    assert_eq!(
                        counts,
                        tests::VisitCount {
                            visit_value: 1,
                            ..Default::default()
                        }
                    );
                }

                #[test]
                fn test_default_visit_slice_small() {
                    let $x = (0..4).map(|i| HelloWorld { id: i }).collect::<Vec<_>>();
                    let hellos = $b;

                    assert_eq!(Listable::size_hint(&hellos), (4, Some(4)));

                    let counts = tests::visit_counts(&hellos);
                    assert_eq!(
                        counts,
                        tests::VisitCount {
                            visit_value: 4,
                            ..Default::default()
                        }
                    );

                    let mut counts = tests::VisitCount::default();
                    valuable::visit(&hellos, &mut counts);
                    assert_eq!(
                        counts,
                        tests::VisitCount {
                            visit_value: 1,
                            ..Default::default()
                        }
                    );

                    let mut visit = VisitList::default();
                    hellos.visit(&mut visit);
                    assert_eq!(visit.0, 4);
                }

                #[test]
                fn test_default_visit_slice_big_pow_2() {
                    let $x = (0..1024).map(|i| HelloWorld { id: i }).collect::<Vec<_>>();
                    let hellos = $b;

                    assert_eq!(Listable::size_hint(&hellos), (1024, Some(1024)));

                    let counts = tests::visit_counts(&hellos);
                    assert_eq!(
                        counts,
                        tests::VisitCount {
                            visit_value: 1024,
                            ..Default::default()
                        }
                    );

                    let mut counts = tests::VisitCount::default();
                    valuable::visit(&hellos, &mut counts);
                    assert_eq!(
                        counts,
                        tests::VisitCount {
                            visit_value: 1,
                            ..Default::default()
                        }
                    );

                    let mut visit = VisitList::default();
                    hellos.visit(&mut visit);
                    assert_eq!(visit.0, 1024);
                }

                #[test]
                fn test_default_visit_slice_big_odd() {
                    let $x = (0..63).map(|i| HelloWorld { id: i }).collect::<Vec<_>>();
                    let hellos = $b;

                    assert_eq!(Listable::size_hint(&hellos), (63, Some(63)));

                    let counts = tests::visit_counts(&hellos);
                    assert_eq!(
                        counts,
                        tests::VisitCount {
                            visit_value: 63,
                            ..Default::default()
                        }
                    );

                    let mut counts = tests::VisitCount::default();
                    valuable::visit(&hellos, &mut counts);
                    assert_eq!(
                        counts,
                        tests::VisitCount {
                            visit_value: 1,
                            ..Default::default()
                        }
                    );

                    let mut visit = VisitList::default();
                    hellos.visit(&mut visit);
                    assert_eq!(visit.0, 63);
                }
            }
        )*
    }
}

test_default! {
    test_vec => |x| x;
    test_slice => |x| &x[..];
}

macro_rules! test_primitive {
    (
        $(
            $name:ident, $variant:ident(Value::$vvariant:ident): $ty:ty => |$x:ident| $b:block;
        )*
    ) => {
        $(
            mod $name {
                use super::*;

                fn test_iter<'a>(mut i: impl Iterator<Item = Value<'a>>, expect: &[$ty]) {
                    assert_eq!(i.size_hint(), (expect.len(), Some(expect.len())));
                    let mut expect = expect.iter();

                    loop {
                        match (i.next(), expect.next()) {
                            (Some(Value::$vvariant(actual)), Some(expect)) => {
                                // When testing floating-point values, the
                                // actual value will be the exact same float
                                // value as the expected, if everything is
                                // working correctly. So, it's not strictly
                                // necessary to use epsilon comparisons here,
                                // and modifying the macro to use epsilon
                                // comparisons for floats would make it
                                // significantly more complex...
                                #[allow(clippy::float_cmp)]
                                {
                                    assert_eq!(actual, *expect)
                                }
                            }
                            (None, None) => break,
                            _ => panic!(),
                        }
                    }
                }

                struct VisitPrimitive<'a>(&'a [$ty]);

                impl Visit for VisitPrimitive<'_> {
                    fn visit_primitive_slice(&mut self, slice: Slice<'_>) {
                        assert_eq!(slice.len(), self.0.len());

                        // fmt::Debug
                        assert_eq!(
                            format!("{:?}", slice),
                            format!("{:?}", self.0),
                        );

                        // Test the expected variant has been received
                        match slice {
                            Slice::$variant(slice) => {
                                assert_eq!(slice, &self.0[..]);
                            }
                            _ => panic!(),
                        }

                        test_iter(slice.iter(), &self.0);
                        test_iter(IntoIterator::into_iter(&slice), &self.0);
                        test_iter(IntoIterator::into_iter(slice), &self.0);
                    }

                    fn visit_value(&mut self, _: Value<'_>) {
                        unreachable!("not called in this test");
                    }
                }

                #[test]
                fn test_empty() {
                    let empty: Vec<$ty> = vec![];

                    assert_eq!(Listable::size_hint(&empty), (0, Some(0)));

                    let counts = tests::visit_counts(&empty);
                    assert_eq!(counts, tests::VisitCount { visit_primitive_slice: 1, .. Default::default() });
                }

                #[test]
                fn test_slices() {
                    fn do_test(listable: &impl Listable, expect: &[$ty]) {
                        assert_eq!(listable.size_hint(), expect.iter().size_hint());

                        let counts = tests::visit_counts(listable);
                        assert_eq!(counts, tests::VisitCount { visit_primitive_slice: 1, .. Default::default() });

                        let mut visit = VisitPrimitive(expect);
                        listable.visit(&mut visit);
                    }

                    for &len in &[4_usize, 10, 30, 32, 63, 64, 100, 1000, 1024] {
                        let vec = (0..len).map(|$x| $b).collect::<Vec<$ty>>();
                        do_test(&vec, &vec);

                        let vec = vec.into_boxed_slice();
                        do_test(&vec, &vec);
                    }
                }
            }
        )*
    };
}

test_primitive! {
    test_bool, Bool(Value::Bool): bool => |x| { x % 2 == 0 };
    test_char, Char(Value::Char): char => |x| { TryFrom::try_from(x as u32).unwrap_or('f') };
    test_f32, F32(Value::F32): f32 => |x| { x as f32 };
    test_f64, F64(Value::F64): f64 => |x| { x as f64 };
    test_i8, I8(Value::I8): i8 => |x| { x as i8 };
    test_i16, I16(Value::I16): i16 => |x| { x as i16 };
    test_i32, I32(Value::I32): i32 => |x| { x as i32 };
    test_i64, I64(Value::I64): i64 => |x| { x as i64 };
    test_i128, I128(Value::I128): i128 => |x| { x as i128 };
    test_isize, Isize(Value::Isize): isize => |x| { x as isize };
    test_str, Str(Value::String): &'static str => |x| { crate::leak(format!("{}", x)) };
    test_string, String(Value::String): String => |x| { format!("{}", x) };
    test_u8, U8(Value::U8): u8 => |x| { x as u8 };
    test_u16, U16(Value::U16): u16 => |x| { x as u16 };
    test_u32, U32(Value::U32): u32 => |x| { x as u32 };
    test_u64, U64(Value::U64): u64 => |x| { x as u64 };
    test_u128, U128(Value::U128): u128 => |x| { x as u128 };
    test_usize, Usize(Value::Usize): usize => |x| { x as usize };
    // test_unit, Unit: () => |_x| { () };
}

fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
