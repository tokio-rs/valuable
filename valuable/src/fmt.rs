use crate::{Valuable, Value, Visit};
use core::fmt;

/// A [`Valuable`] sub-type that can be borrowed as something that implements
/// [`core::fmt::Display`].
///
/// Some types, such as the standard library's [`std::path::Path`] and
/// [`std::path::PathBuf`], do not have [`fmt::Display`] implementations,
/// but provide a method that borrows the type as a wrapper that implements
/// [`Display`]. A `Path`, therefore, cannot be recorded as a `&dyn
/// fmt::Display`, because it does not implement [`fmt::Display`] itself, and it
/// cannot
pub trait Displayable {
    /// Records a `&dyn fmt::Display` value with the provided [visitor].
    ///
    /// This function's implementation is generally expected to call the
    /// visitor's [`Visit::visit_display`] method.
    ///
    /// [visitor]: Visit
    fn visit_display(&self, visit: &mut dyn Visit);
}

impl fmt::Display for dyn Displayable + '_ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct FmtVisitor<'a, 'f> {
            f: &'a mut fmt::Formatter<'f>,
            result: fmt::Result,
        }
        impl<'a, 'f> Visit for FmtVisitor<'a, 'f> {
            fn visit_value(&mut self, _: Value<'_>) {
                unreachable!("`FmtVisitor::visit_value` should never be called")
            }

            #[inline]
            fn visit_display(&mut self, value: &dyn fmt::Display) {
                self.result = value.fmt(self.f);
            }
        }
        let mut visitor = FmtVisitor { f, result: Ok(()) };
        self.visit_display(&mut visitor);
        visitor.result
    }
}

impl Displayable for &'_ dyn fmt::Display {
    #[inline]
    fn visit_display(&self, visit: &mut dyn Visit) {
        visit.visit_display(self);
    }
}

impl Valuable for &'_ dyn fmt::Display {
    #[inline]
    fn as_value(&self) -> Value<'_> {
        Value::Displayable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_display(self);
    }
}

// ==== Displayable impls ====

macro_rules! impl_displayable {
    ($($(#[$meta:meta])* $ty:ty),+ $(,)?) => {
        $(
            $(#[$meta])*
            impl Displayable for $ty {
                #[inline]
                fn visit_display(&self, visit: &mut dyn Visit) {
                    visit.visit_display(self);
                }
            }
        )+
    };
}

impl_displayable! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
    bool,
    char,
    &'_ str,
    fmt::Arguments<'_>,
    #[cfg(feature = "std")]
    &'_ dyn std::error::Error,
}

#[cfg(feature = "std")]
impl Displayable for &std::path::Path {
    #[inline]
    fn visit_display(&self, visit: &mut dyn Visit) {
        visit.visit_display(&self.display());
    }
}

#[cfg(feature = "std")]
impl Displayable for std::path::PathBuf {
    #[inline]
    fn visit_display(&self, visit: &mut dyn Visit) {
        visit.visit_display(&self.display());
    }
}

impl Valuable for fmt::Arguments<'_> {
    #[inline]
    fn as_value(&self) -> Value<'_> {
        Value::Displayable(self)
    }

    #[inline]
    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_display(self);
    }
}
