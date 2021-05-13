use crate::*;

use core::fmt;

macro_rules! value {
    (
        $(
            $(#[$attrs:meta])*
            $variant:ident($ty:ty),
        )*
    ) => {
        /// Any Rust value
        ///
        /// The `Value` enum is used to pass single values to the
        /// [visitor][`Visit`]. Primitive types are enumerated and other types
        /// are represented at trait objects.
        ///
        /// Values are converted to `Value` instances using
        /// [`Valuable::as_value()`].
        ///
        /// # Examples
        ///
        /// Convert a primitive type
        ///
        /// ```
        /// use valuable::{Value, Valuable};
        ///
        /// let num = 123;
        /// let val = num.as_value();
        ///
        /// assert!(matches!(val, Value::I32(v) if v == 123));
        /// ```
        ///
        /// Converting a struct
        ///
        /// ```
        /// use valuable::{Value, Valuable};
        ///
        /// #[derive(Valuable, Debug)]
        /// struct HelloWorld {
        ///     message: String,
        /// }
        ///
        /// let hello = HelloWorld {
        ///     message: "greetings".to_string(),
        /// };
        ///
        /// let val = hello.as_value();
        ///
        /// assert!(matches!(val, Value::Structable(_v)));
        ///
        /// // The Value `Debug` output matches the struct's
        /// assert_eq!(
        ///     format!("{:?}", val),
        ///     format!("{:?}", hello),
        /// );
        /// ```
        ///
        /// [visitor]: Visit
        #[non_exhaustive]
        #[derive(Clone, Copy)]
        pub enum Value<'a> {
            $(
                $(#[$attrs])*
                $variant($ty),
            )*
            Unit, // TODO: None?
        }

        $(
            $(#[$attrs])*
            impl<'a> From<$ty> for Value<'a> {
                fn from(src: $ty) -> Value<'a> {
                    Value::$variant(src)
                }
            }
        )*

        impl<'a> From<()> for Value<'a> {
            fn from(_: ()) -> Value<'a> {
                Value::Unit
            }
        }

        impl fmt::Debug for Value<'_> {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                use Value::*;

                // Doc comments are expanded into the branch arms, which results
                // in a warning. It isn't a big deal, so silence it.
                #[allow(unused_doc_comments)]
                match self {
                    $(
                        $(#[$attrs])*
                        $variant(v) => fmt::Debug::fmt(v, fmt),
                    )*
                    Unit => ().fmt(fmt),
                }
            }
        }
    }
}

value! {
    /// A Rust `bool` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::Bool(true);
    /// ```
    Bool(bool),

    /// A Rust `char` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::Char('h');
    /// ```
    Char(char),

    /// A Rust `f32` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::F32(3.1415);
    /// ```
    F32(f32),

    /// A Rust `f64` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::F64(3.1415);
    /// ```
    F64(f64),

    /// A Rust `i8` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::I8(42);
    /// ```
    I8(i8),

    /// A Rust `i16` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::I16(42);
    /// ```
    I16(i16),

    /// A Rust `i32` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::I32(42);
    /// ```
    I32(i32),

    /// A Rust `i64` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::I64(42);
    /// ```
    I64(i64),

    /// A Rust `i128` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::I128(42);
    /// ```
    I128(i128),

    /// A Rust `isize` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::Isize(42);
    /// ```
    Isize(isize),

    /// A Rust `&str` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::String("hello");
    /// ```
    String(&'a str),

    /// A Rust `u8` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::U8(42);
    /// ```
    U8(u8),

    /// A Rust `u16` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::U16(42);
    /// ```
    U16(u16),

    /// A Rust `u32` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::U32(42);
    /// ```
    U32(u32),

    /// A Rust `u64` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::U64(42);
    /// ```
    U64(u64),

    /// A Rust `u128` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::U128(42);
    /// ```
    U128(u128),

    /// A Rust `usize` value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let v = Value::Usize(42);
    /// ```
    Usize(usize),

    /// A Rust error value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    /// use std::io;
    ///
    /// let err: io::Error = io::ErrorKind::Other.into();
    /// let v = Value::Error(&err);
    /// ```
    #[cfg(feature = "std")]
    Error(&'a dyn std::error::Error),

    /// A Rust list value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    ///
    /// let vals = vec![1, 2, 3, 4, 5];
    /// let v = Value::Listable(&vals);
    /// ```
    Listable(&'a dyn Listable),

    /// A Rust map value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Value;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("foo", 1);
    /// map.insert("bar", 2);
    ///
    /// let v = Value::Mappable(&map);
    /// ```
    Mappable(&'a dyn Mappable),

    /// A Rust struct value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Value, Valuable};
    ///
    /// #[derive(Valuable)]
    /// struct MyStruct {
    ///     field: u32,
    /// }
    ///
    /// let my_struct = MyStruct {
    ///     field: 123,
    /// };
    ///
    /// let v = Value::Structable(&my_struct);
    /// ```
    Structable(&'a dyn Structable),

    /// A Rust enum value
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Value, Valuable};
    ///
    /// #[derive(Valuable)]
    /// enum MyEnum {
    ///     Foo,
    ///     Bar,
    /// }
    ///
    /// let my_enum = MyEnum::Foo;
    /// let v = Value::Enumerable(&my_enum);
    /// ```
    Enumerable(&'a dyn Enumerable),
}

impl Valuable for Value<'_> {
    fn as_value(&self) -> Value<'_> {
        self.clone()
    }

    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_value(self.clone());
    }
}

impl Default for Value<'_> {
    fn default() -> Self {
        Value::Unit
    }
}

macro_rules! convert {
    (
        $(
            $ty:ty => $as:ident,
        )*
    ) => {
        impl<'a> Value<'a> {
            pub fn as_bool(&self) -> Option<bool> {
                match *self {
                    Value::Bool(v) => Some(v),
                    _ => None,
                }
            }

            pub fn as_char(&self) -> Option<char> {
                match *self {
                    Value::Char(v) => Some(v),
                    _ => None,
                }
            }

            pub fn as_f32(&self) -> Option<f32> {
                match *self {
                    Value::F32(v) => Some(v),
                    _ => None,
                }
            }

            pub fn as_f64(&self) -> Option<f64> {
                match *self {
                    Value::F64(v) => Some(v),
                    _ => None,
                }
            }

            $(
                pub fn $as(&self) -> Option<$ty> {
                    use Value::*;
                    use core::convert::TryInto;

                    match *self {
                        I8(v) => v.try_into().ok(),
                        I16(v) => v.try_into().ok(),
                        I32(v) => v.try_into().ok(),
                        I64(v) => v.try_into().ok(),
                        I128(v) => v.try_into().ok(),
                        Isize(v) => v.try_into().ok(),
                        U8(v) => v.try_into().ok(),
                        U16(v) => v.try_into().ok(),
                        U32(v) => v.try_into().ok(),
                        U64(v) => v.try_into().ok(),
                        U128(v) => v.try_into().ok(),
                        Usize(v) => v.try_into().ok(),
                        _ => None,
                    }
                }
            )*

            pub fn as_str(&self) -> Option<&str> {
                match *self {
                    Value::String(v) => Some(v),
                    _ => None,
                }
            }

            #[cfg(feature = "std")]
            pub fn as_error(&self) -> Option<&dyn std::error::Error> {
                match *self {
                    Value::Error(v) => Some(v),
                    _ => None,
                }
            }

            pub fn as_listable(&self) -> Option<&dyn Listable> {
                match *self {
                    Value::Listable(v) => Some(v),
                    _ => None,
                }
            }

            pub fn as_mappable(&self) -> Option<&dyn Mappable> {
                match *self {
                    Value::Mappable(v) => Some(v),
                    _ => None,
                }
            }

            pub fn as_structable(&self) -> Option<&dyn Structable> {
                match *self {
                    Value::Structable(v) => Some(v),
                    _ => None,
                }
            }

            pub fn as_enumerable(&self) -> Option<&dyn Enumerable> {
                match *self {
                    Value::Enumerable(v) => Some(v),
                    _ => None,
                }
            }
        }
    }
}

convert! {
    i8 => as_i8,
    i16 => as_i16,
    i32 => as_i32,
    i64 => as_i64,
    i128 => as_i128,
    isize => as_isize,
    u8 => as_u8,
    u16 => as_u16,
    u32 => as_u32,
    u64 => as_u64,
    u128 => as_u128,
    usize => as_usize,
}
