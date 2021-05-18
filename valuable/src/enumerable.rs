use crate::field::*;
use crate::*;

#[cfg(feature = "alloc")]
use alloc::format;
use core::fmt;

/// An enum-like [`Valuable`] sub-type.
///
/// Implemented by [`Valuable`] types that have an enum-like shape. Fields may
/// be named or unnamed (tuple). Values that implement `Enumerable` must return
/// [`Value::Enumerable`] from their [`Valuable::as_value`] implementation.
///
/// # Inspecting
///
/// The [`variant()`] method returns the `Enumerable` instance's variant. The
/// `Enumerable` may also have unnamed fields (tuple) or named fields.
/// Inspecting the field values is done by visiting the enum. When visiting an
/// `Enumerable`, either the [`visit_named_fields()`] or the
/// [`visit_unnamed_fields()`] methods of [`Visit`] are called. Each method may
/// be called multiple times per `Enumerable`, but the two methods are never
/// mixed.
///
/// ```
/// use valuable::{Valuable, Value, Visit};
///
/// #[derive(Valuable)]
/// enum MyEnum {
///     Foo,
///     Bar(u32),
/// }
///
/// struct PrintVariant;
///
/// impl Visit for PrintVariant {
///     fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
///         for value in values {
///             println!(" - {:?}", value);
///         }
///     }
///
///     fn visit_value(&mut self, value: Value<'_>) {
///         match value {
///             Value::Enumerable(v) => {
///                 println!("{}", v.variant().name());
///                 v.visit(self)
///             }
///             _ => {}
///         }
///     }
/// }
///
/// let my_enum = MyEnum::Bar(123);
///
/// valuable::visit(&my_enum, &mut PrintVariant);
/// ```
///
/// If the enum is **statically** defined, then all variants, and variant fields
/// are known ahead of time and may be accessed via the [`EnumDef`] instance
/// returned by [`definition()`].
///
/// # Implementing
///
/// Implementing `Enumerable` is usually done by adding `#[derive(Valuable)]` to
/// a Rust `enum` definition.
///
/// ```
/// use valuable::{Valuable, Enumerable, EnumDef};
///
/// #[derive(Valuable)]
/// enum MyEnum {
///     Foo,
///     Bar(u32),
/// }
///
/// let my_enum = MyEnum::Bar(123);
///
/// let variants = match my_enum.definition() {
///     EnumDef::Static { name, variants, .. } => {
///         assert_eq!("MyEnum", name);
///         variants
///     }
///     _ => unreachable!(),
/// };
///
/// assert_eq!(2, variants.len());
/// assert_eq!("Foo", variants[0].name());
/// assert!(variants[0].fields().is_unnamed());
/// ```
pub trait Enumerable: Valuable {
    fn definition(&self) -> EnumDef<'_>;

    fn variant(&self) -> Variant<'_>;
}

#[non_exhaustive]
pub enum EnumDef<'a> {
    #[non_exhaustive]
    Static {
        name: &'static str,
        variants: &'static [VariantDef<'static>],
    },

    #[non_exhaustive]
    Dynamic {
        name: &'a str,
        variants: &'a [VariantDef<'a>],
    },
}

pub struct VariantDef<'a> {
    /// Variant name
    name: &'a str,

    fields: Fields<'a>,
}

pub enum Variant<'a> {
    Static(&'static VariantDef<'static>),
    Dynamic(VariantDef<'a>),
}

impl<'a> EnumDef<'a> {
    pub const fn new_static(
        name: &'static str,
        variants: &'static [VariantDef<'static>],
    ) -> EnumDef<'a> {
        EnumDef::Static { name, variants }
    }

    pub const fn new_dynamic(name: &'a str, variants: &'a [VariantDef<'a>]) -> EnumDef<'a> {
        EnumDef::Dynamic { name, variants }
    }

    pub fn name(&self) -> &str {
        match self {
            EnumDef::Static { name, .. } => name,
            EnumDef::Dynamic { name, .. } => name,
        }
    }

    pub fn variants(&self) -> &[VariantDef<'_>] {
        match self {
            EnumDef::Static { variants, .. } => variants,
            EnumDef::Dynamic { variants, .. } => variants,
        }
    }

    pub fn is_static(&self) -> bool {
        matches!(self, EnumDef::Static { .. })
    }

    pub fn is_dynamic(&self) -> bool {
        matches!(self, EnumDef::Dynamic { .. })
    }
}

impl<'a> VariantDef<'a> {
    pub const fn new(name: &'a str, fields: Fields<'a>) -> VariantDef<'a> {
        VariantDef { name, fields }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn fields(&self) -> &Fields<'_> {
        &self.fields
    }
}

impl Variant<'_> {
    pub fn name(&self) -> &str {
        match self {
            Variant::Static(v) => v.name(),
            Variant::Dynamic(v) => v.name(),
        }
    }

    pub fn is_named_fields(&self) -> bool {
        match self {
            Variant::Static(v) => v.fields().is_named(),
            Variant::Dynamic(v) => v.fields().is_named(),
        }
    }

    pub fn is_unnamed_fields(&self) -> bool {
        !self.is_named_fields()
    }
}

impl fmt::Debug for dyn Enumerable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant = self.variant();
        #[cfg(feature = "alloc")]
        let name = format!("{}::{}", self.definition().name(), variant.name());
        #[cfg(not(feature = "alloc"))]
        let name = variant.name();

        if variant.is_named_fields() {
            struct DebugEnum<'a, 'b> {
                fmt: fmt::DebugStruct<'a, 'b>,
            }

            let mut debug = DebugEnum {
                fmt: fmt.debug_struct(&name),
            };

            impl Visit for DebugEnum<'_, '_> {
                fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
                    for (field, value) in named_values {
                        self.fmt.field(field.name(), value);
                    }
                }
            }

            self.visit(&mut debug);

            debug.fmt.finish()
        } else {
            struct DebugEnum<'a, 'b> {
                fmt: fmt::DebugTuple<'a, 'b>,
            }

            let mut debug = DebugEnum {
                fmt: fmt.debug_tuple(&name),
            };

            impl Visit for DebugEnum<'_, '_> {
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

impl<E: ?Sized + Enumerable> Enumerable for &E {
    fn definition(&self) -> EnumDef<'_> {
        E::definition(*self)
    }

    fn variant(&self) -> Variant<'_> {
        E::variant(*self)
    }
}

impl<E: ?Sized + Enumerable> Enumerable for &mut E {
    fn definition(&self) -> EnumDef<'_> {
        E::definition(&**self)
    }

    fn variant(&self) -> Variant<'_> {
        E::variant(&**self)
    }
}

#[cfg(feature = "alloc")]
impl<E: ?Sized + Enumerable> Enumerable for alloc::boxed::Box<E> {
    fn definition(&self) -> EnumDef<'_> {
        E::definition(&**self)
    }

    fn variant(&self) -> Variant<'_> {
        E::variant(&**self)
    }
}

#[cfg(feature = "alloc")]
impl<E: ?Sized + Enumerable> Enumerable for alloc::rc::Rc<E> {
    fn definition(&self) -> EnumDef<'_> {
        E::definition(&**self)
    }

    fn variant(&self) -> Variant<'_> {
        E::variant(&**self)
    }
}

#[cfg(not(valuable_no_atomic_cas))]
#[cfg(feature = "alloc")]
impl<E: ?Sized + Enumerable> Enumerable for alloc::sync::Arc<E> {
    fn definition(&self) -> EnumDef<'_> {
        E::definition(&**self)
    }

    fn variant(&self) -> Variant<'_> {
        E::variant(&**self)
    }
}
