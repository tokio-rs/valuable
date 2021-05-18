use crate::field::*;
use crate::*;

use core::fmt;

pub trait Structable: Valuable {
    fn definition(&self) -> StructDef<'_>;
}

/// A struct's name, fields, and other struct-level information.
///
/// Returned by [`Structable::definition()`], `StructDef` provides the caller
/// with information about the struct's definition.
///
/// [`Structable::definition()`]: Structable::definition
#[non_exhaustive]
pub enum StructDef<'a> {
    /// The struct is statically-defined, all fields are known ahead of time.
    ///
    /// Most `Structable` definitions for Rust struct types will be `StructDef::Static`.
    ///
    /// # Examples
    ///
    /// A statically defined struct
    ///
    /// ```
    /// use valuable::{Fields, Valuable, Structable, StructDef};
    ///
    /// #[derive(Valuable)]
    /// struct MyStruct {
    ///     foo: &'static str,
    /// }
    ///
    /// let my_struct = MyStruct { foo: "Hello" };
    /// let fields = match my_struct.definition() {
    ///     StructDef::Static { name, fields, ..} => {
    ///         assert_eq!("MyStruct", name);
    ///         fields
    ///     }
    ///     _ => unreachable!(),
    /// };
    ///
    /// match fields {
    ///     Fields::Named(named_fields) => {
    ///         assert_eq!(1, named_fields.len());
    ///         assert_eq!("foo", named_fields[0].name());
    ///     }
    ///     _ => unreachable!(),
    /// }
    /// ```
    #[non_exhaustive]
    Static {
        /// The struct's name.
        name: &'static str,

        /// The struct's fields.
        fields: Fields<'static>,
    },

    /// The struct is dynamically-defined, not all fields are known ahead of time.
    ///
    /// # Examples
    ///
    /// The struct stores field values in a `HashMap`.
    ///
    /// ```
    /// use valuable::{Fields, NamedField, NamedValues, Structable, StructDef, Value, Valuable, Visit};
    /// use std::collections::HashMap;
    ///
    /// /// A dynamic struct
    /// struct Dyn {
    ///     // The struct name
    ///     name: String,
    ///
    ///     // Named values.
    ///     values: HashMap<String, Box<dyn Valuable>>,
    /// }
    ///
    /// impl Valuable for Dyn {
    ///     fn as_value(&self) -> Value<'_> {
    ///         Value::Structable(self)
    ///     }
    ///
    ///     fn visit(&self, visit: &mut dyn Visit) {
    ///         // This could be optimized to batch some.
    ///         for (field, value) in self.values.iter() {
    ///             visit.visit_named_fields(&NamedValues::new(
    ///                 &[NamedField::new(field)],
    ///                 &[value.as_value()],
    ///             ));
    ///         }
    ///     }
    /// }
    ///
    /// impl Structable for Dyn {
    ///     fn definition(&self) -> StructDef<'_> {
    ///         StructDef::new_dynamic(&self.name, Fields::Named(&[]))
    ///     }
    /// }
    /// ```
    #[non_exhaustive]
    Dynamic {
        /// The struct's name
        name: &'a str,

        /// The struct's fields.
        fields: Fields<'a>,
    },
}

impl fmt::Debug for dyn Structable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let def = self.definition();

        if def.fields().is_named() {
            struct DebugStruct<'a, 'b> {
                fmt: fmt::DebugStruct<'a, 'b>,
            }

            let mut debug = DebugStruct {
                fmt: fmt.debug_struct(def.name()),
            };

            impl Visit for DebugStruct<'_, '_> {
                fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
                    for (field, value) in named_values.entries() {
                        self.fmt.field(field.name(), value);
                    }
                }
            }

            self.visit(&mut debug);

            debug.fmt.finish()
        } else {
            struct DebugStruct<'a, 'b> {
                fmt: fmt::DebugTuple<'a, 'b>,
            }

            let mut debug = DebugStruct {
                fmt: fmt.debug_tuple(def.name()),
            };

            impl Visit for DebugStruct<'_, '_> {
                fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
                    for value in values {
                        self.fmt.field(value);
                    }
                }
            }

            self.visit(&mut debug);

            debug.fmt.finish()
        }
    }
}

impl<'a> StructDef<'a> {
    /// Create a new [`StructDef::Static`] instance.
    ///
    /// This should be used when a struct's fields are fixed and known ahead of time.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_static("Foo", Fields::Unnamed);
    /// ```
    pub const fn new_static(name: &'static str, fields: Fields<'static>) -> StructDef<'a> {
        StructDef::Static { name, fields }
    }

    /// Create a new [`StructDef::Dyanmic`] instance.
    ///
    /// This is used when the struct's fields may vary at runtime.
    /// # Examples
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_dynamic("Foo", Fields::Unnamed);
    /// ```
    pub const fn new_dynamic(name: &'a str, fields: Fields<'a>) -> StructDef<'a> {
        StructDef::Dynamic { name, fields }
    }

    /// Returns the struct's name
    ///
    /// # Examples
    ///
    /// With a static struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_static("Foo", Fields::Unnamed);
    /// assert_eq!("Foo", def.name());
    /// ```
    ///
    /// With a dynamic struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_dynamic("Foo", Fields::Unnamed);
    /// assert_eq!("Foo", def.name());
    /// ```
    pub fn name(&self) -> &str {
        match self {
            StructDef::Static { name, .. } => name,
            StructDef::Dynamic { name, .. } => name,
        }
    }

    /// Returns the struct's fields
    ///
    /// # Examples
    ///
    /// With a static struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_static("Foo", Fields::Unnamed);
    /// assert!(matches!(def.fields(), Fields::Unnamed));
    /// ```
    ///
    /// With a dynamic struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_dynamic("Foo", Fields::Unnamed);
    /// assert!(matches!(def.fields(), Fields::Unnamed));
    /// ```
    pub fn fields(&self) -> &Fields<'_> {
        match self {
            StructDef::Static { fields, .. } => fields,
            StructDef::Dynamic { fields, .. } => fields,
        }
    }

    /// Returns `true` if the struct is [statically defined](StructDef::Static).
    ///
    /// # Examples
    ///
    /// With a static struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_static("Foo", Fields::Unnamed);
    /// assert!(def.is_static());
    /// ```
    ///
    /// With a dynamic struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_dynamic("Foo", Fields::Unnamed);
    /// assert!(!def.is_static());
    /// ```
    pub fn is_static(&self) -> bool {
        matches!(self, StructDef::Static { .. })
    }

    /// Returns `true` if the struct is [dynamically defined](StructDef::Static).
    ///
    /// # Examples
    ///
    /// With a static struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_static("Foo", Fields::Unnamed);
    /// assert!(!def.is_dynamic());
    /// ```
    ///
    /// With a dynamic struct
    ///
    /// ```
    /// use valuable::{StructDef, Fields};
    ///
    /// let def = StructDef::new_dynamic("Foo", Fields::Unnamed);
    /// assert!(def.is_dynamic());
    /// ```
    pub fn is_dynamic(&self) -> bool {
        matches!(self, StructDef::Dynamic { .. })
    }
}

impl<S: ?Sized + Structable> Structable for &S {
    fn definition(&self) -> StructDef<'_> {
        S::definition(*self)
    }
}

impl<S: ?Sized + Structable> Structable for &mut S {
    fn definition(&self) -> StructDef<'_> {
        S::definition(&**self)
    }
}

#[cfg(feature = "alloc")]
impl<S: ?Sized + Structable> Structable for alloc::boxed::Box<S> {
    fn definition(&self) -> StructDef<'_> {
        S::definition(&**self)
    }
}

#[cfg(feature = "alloc")]
impl<S: ?Sized + Structable> Structable for alloc::rc::Rc<S> {
    fn definition(&self) -> StructDef<'_> {
        S::definition(&**self)
    }
}

#[cfg(not(valuable_no_atomic_cas))]
#[cfg(feature = "alloc")]
impl<S: ?Sized + Structable> Structable for alloc::sync::Arc<S> {
    fn definition(&self) -> StructDef<'_> {
        S::definition(&**self)
    }
}
