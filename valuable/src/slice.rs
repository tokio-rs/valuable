use crate::*;

use core::fmt;
use core::iter::FusedIterator;

macro_rules! slice {
    (
        $(
            $(#[$attrs:meta])*
            $variant:ident($ty:ty),
        )*
    ) => {
        #[non_exhaustive]
        pub enum Slice<'a> {
            $(
                $(#[$attrs])*
                $variant(&'a [$ty]),
            )*
        }

        pub struct Iter<'a>(IterKind<'a>);

        enum IterKind<'a> {
            $(
                $(#[$attrs])*
                $variant(core::slice::Iter<'a, $ty>),
            )*
        }

        impl<'a> Slice<'a> {
            pub fn len(&self) -> usize {
                match self {
                    $(
                        $(#[$attrs])*
                        Slice::$variant(s) => s.len(),
                    )*
                }
            }

            pub fn iter(&self) -> Iter<'a> {
                self.into_iter()
            }
        }

        impl<'a> IntoIterator for Slice<'a> {
            type Item = Value<'a>;
            type IntoIter = Iter<'a>;

            fn into_iter(self) -> Self::IntoIter {
                (&self).into_iter()
            }
        }

        impl<'a> IntoIterator for &'_ Slice<'a> {
            type Item = Value<'a>;
            type IntoIter = Iter<'a>;

            fn into_iter(self) -> Self::IntoIter {
                Iter(match self {
                    $(
                        $(#[$attrs])*
                        Slice::$variant(s) => IterKind::$variant(s.iter()),
                    )*
                })
            }
        }

        impl fmt::Debug for Slice<'_> {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                use Slice::*;

                let mut d = fmt.debug_list();

                match *self {
                    $(
                        $(#[$attrs])*
                        $variant(v) => d.entries(v),
                    )*
                };

                d.finish()
            }
        }

        impl<'a> Iterator for Iter<'a> {
            type Item = Value<'a>;

            fn size_hint(&self) -> (usize, Option<usize>) {
                use IterKind::*;

                match &self.0 {
                    $(
                        $(#[$attrs])*
                        $variant(v) => v.size_hint(),
                    )*
                }
            }

            fn next(&mut self) -> Option<Value<'a>> {
                use IterKind::*;

                match &mut self.0 {
                    $(
                        $(#[$attrs])*
                        $variant(v) => v.next().map(Valuable::as_value),
                    )*
                }
            }
        }

        impl DoubleEndedIterator for Iter<'_> {
            fn next_back(&mut self) -> Option<Self::Item> {
                use IterKind::*;

                match &mut self.0 {
                    $(
                        $(#[$attrs])*
                        $variant(v) => v.next_back().map(Valuable::as_value),
                    )*
                }
            }
        }

        impl ExactSizeIterator for Iter<'_> {
            fn len(&self) -> usize {
                use IterKind::*;

                match &self.0 {
                    $(
                        $(#[$attrs])*
                        $variant(v) => v.len(),
                    )*
                }
            }
        }

        impl FusedIterator for Iter<'_> {}
    }
}

slice! {
    Bool(bool),
    Char(char),
    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    Str(&'a str),
    #[cfg(feature = "alloc")]
    String(alloc::string::String),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    Unit(()),
}
