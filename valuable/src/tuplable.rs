use crate::{Valuable, Value, Visit};

use core::fmt;

/// TODO
pub trait Tuplable: Valuable {
    /// TODO
    fn definition(&self) -> TupleDef;
}

/// TODO
#[derive(Debug)]
#[non_exhaustive]
pub enum TupleDef {
    /// TODO
    #[non_exhaustive]
    Static {
        /// TODO
        fields: usize,
    },
    /// TODO
    #[non_exhaustive]
    Dynamic {
        /// TODO
        fields: (usize, Option<usize>),
    },
}

macro_rules! tuple_impls {
    (
        $( $len:expr => ( $($n:tt $name:ident)+ ) )+
    ) => {
        $(
            impl<$($name),+> Valuable for ($($name,)+)
            where
                $($name: Valuable,)+
            {
                fn as_value(&self) -> Value<'_> {
                    unimplemented!()
                }

                fn visit(&self, visit: &mut dyn Visit) {
                    visit.visit_unnamed_fields(&[
                        $(
                            self.$n.as_value(),
                        )+
                    ]);
                }
            }

            impl<$($name),+> Tuplable for ($($name,)+)
            where
                $($name: Valuable,)+
            {
                fn definition(&self) -> TupleDef {
                    TupleDef::Static { fields: $len }
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (0 T0)
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

impl fmt::Debug for dyn Tuplable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DebugTuple<'a, 'b> {
            fmt: fmt::DebugTuple<'a, 'b>,
        }

        impl Visit for DebugTuple<'_, '_> {
            fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
                for value in values {
                    self.fmt.field(value);
                }
            }

            fn visit_value(&mut self, _: Value<'_>) {
                unimplemented!()
            }
        }

        let mut debug = DebugTuple {
            fmt: fmt.debug_tuple(""),
        };

        self.visit(&mut debug);
        debug.fmt.finish()
    }
}

impl TupleDef {
    /// Create a new [`TupleDef::Static`] instance
    ///
    /// This should be used when the tuple's fields are fixed and known ahead of time.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::TupleDef;
    ///
    /// let def = TupleDef::new_static(2);
    /// ```
    pub const fn new_static(fields: usize) -> TupleDef {
        TupleDef::Static { fields }
    }

    /// Create a new [`TupleDef::Dynamic`] instance.
    ///
    /// TODO
    pub const fn new_dynamic(fields: (usize, Option<usize>)) -> TupleDef {
        TupleDef::Dynamic { fields }
    }

    /// Returns `true` if the tuple is [statically defined](TupleDef::Static).
    ///
    /// # Examples
    ///
    /// With a static tuple
    ///
    /// ```
    /// use valuable::TupleDef;
    ///
    /// let def = TupleDef::new_static(2);
    /// assert!(def.is_static());
    /// ```
    ///
    /// With a dynamic tuple
    ///
    /// ```
    /// use valuable::TupleDef;
    ///
    /// let def = TupleDef::new_dynamic((2, None));
    /// assert!(!def.is_static());
    /// ```
    pub fn is_static(&self) -> bool {
        matches!(self, TupleDef::Static { .. })
    }

    /// Returns `true` if the tuple is [dynamically defined](TupleDef::Dynamic).
    ///
    /// # Examples
    ///
    /// With a static tuple
    ///
    /// ```
    /// use valuable::TupleDef;
    ///
    /// let def = TupleDef::new_static(2);
    /// assert!(!def.is_dynamic());
    /// ```
    ///
    /// With a dynamic tuple
    ///
    /// ```
    /// use valuable::TupleDef;
    ///
    /// let def = TupleDef::new_dynamic((2, None));
    /// assert!(def.is_dynamic());
    /// ```
    pub fn is_dynamic(&self) -> bool {
        matches!(self, TupleDef::Dynamic { .. })
    }
}
