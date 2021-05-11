use tests::*;
use valuable::*;

macro_rules! test_mappable {
    ($($name:ident => $ty:ident,)*) => {
        $(
            mod $name {
                use super::*;

                #[test]
                fn test_empty() {
                    let map = std::collections::$ty::<(), ()>::new();

                    assert_eq!(Mappable::size_hint(&map), (0, Some(0)));

                    let counts = visit_counts(&map);
                    assert_eq!(
                        counts,
                        Default::default()
                    );

                    assert_eq!(
                        format!("{:?}", map.as_value()),
                        format!("{:?}", map)
                    );
                }

                #[test]
                fn test_full() {
                    let mut map = std::collections::$ty::new();
                    map.insert("foo", 123);
                    map.insert("bar", 456);

                    assert_eq!(Mappable::size_hint(&map), (2, Some(2)));

                    let counts = visit_counts(&map);
                    assert_eq!(
                        counts,
                        tests::VisitCount {
                            visit_entry: 2,
                            ..Default::default()
                        }
                    );

                    assert_eq!(
                        format!("{:?}", map.as_value()),
                        format!("{:?}", map)
                    );
                }

                #[test]
                fn test_nested_structable() {
                    let mut map = std::collections::$ty::new();
                    map.insert("foo", HelloWorld { id: 1 });
                    map.insert("bar", HelloWorld { id: 2 });
                    map.insert("baz", HelloWorld { id: 3 });

                    assert_eq!(Mappable::size_hint(&map), (3, Some(3)));

                    let counts = visit_counts(&map);
                    assert_eq!(
                        counts,
                        tests::VisitCount {
                            visit_entry: 3,
                            ..Default::default()
                        }
                    );

                    assert_eq!(
                        format!("{:?}", map.as_value()),
                        format!("{:?}", map)
                    );
                }
            }
        )*
    };
}

test_mappable! {
    hash_map => HashMap,
    btree_map => BTreeMap,
}
