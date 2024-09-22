use core::fmt::{self, Debug, Display, Formatter, Write};

use crate::{Slice, Valuable, Value};

/// A [`Valuable`] sub-type for using ordinarily non-`valuable` types by
/// rendering them to a string with [`Debug`] or [`Display`].
///
/// This is most useful when defining a [`Structable`] value that includes
/// fields of types where [`Valuable`] cannot be implemented like types
/// contained in external crates.
///
/// ```
/// use valuable::{Valuable, Value, Visit, Renderable};
///
/// #[derive(Debug)]
/// struct NotValuable {
///     foo: u32,
///     bar: u32,
/// }
///
/// struct Render(String);
///
/// impl Visit for Render {
///     fn visit_value(&mut self, value: Value<'_>) {
///         let Value::Renderable(v) = value else { return };
///          self.0 = v.render_to_string();
///     }
/// }
///
/// let my_struct = NotValuable {
///     foo: 123,
///     bar: 456,
/// };
///
/// let mut renderer = Render(String::default());
///
/// // Render it plain
///
/// valuable::visit(&Renderable::Debug(&my_struct), &mut renderer);
/// assert_eq!(renderer.0, "NotValuable { foo: 123, bar: 456 }");
///
/// // Or render it pretty
/// assert_eq!(Renderable::Debug(&my_struct).render_to_string_with_prettiness(true),
/// "NotValuable {
///     foo: 123,
///     bar: 456,
/// }");
///
/// ```
#[derive(Clone, Copy)]
pub enum Renderable<'a> {
    /// Renderable sub-type that is rendered via its [`Debug`] implementation
    /// ```
    /// use valuable::{Valuable, Value, Visit, Renderable};
    ///
    /// #[derive(Debug)]
    /// struct NotValuable {
    ///     foo: u32,
    ///     bar: u32,
    /// }
    ///
    /// struct Renderer(String);
    ///
    /// impl Visit for Renderer {
    ///     fn visit_value(&mut self, value: Value<'_>) {
    ///         let Value::Renderable(v) = value else { return };
    ///         self.0 = v.render_to_string();
    ///     }
    /// }
    ///
    /// let my_struct = NotValuable {
    ///     foo: 123,
    ///     bar: 456,
    /// };
    ///
    /// let mut renderer = Renderer(String::default());
    ///
    /// valuable::visit(&Renderable::Debug(&my_struct), &mut renderer);
    /// assert_eq!(renderer.0, "NotValuable { foo: 123, bar: 456 }");
    /// ```
    Debug(
        /// The actual type to be rendered with [`Debug::fmt`]
        &'a dyn Debug,
    ),

    /// Renderable sub-type that is rendered via its [`Display`] implementation
    /// ```
    /// use valuable::{Valuable, Value, Visit, Renderable};
    /// use core::fmt;
    ///
    /// struct NotValuable {
    ///     foo: u32,
    ///     bar: u32,
    /// }
    ///
    /// struct Renderer(String);
    ///
    /// impl Visit for Renderer {
    ///     fn visit_value(&mut self, value: Value<'_>) {
    ///         let Value::Renderable(v) = value else { return };
    ///          self.0 = v.render_to_string();
    ///     }
    /// }
    ///
    /// let my_struct = NotValuable {
    ///     foo: 123,
    ///     bar: 456,
    /// };
    ///
    /// impl fmt::Display for NotValuable {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "[Foo: {}, Bar: {}]", &self.foo, &self.bar)
    ///     }
    /// }
    ///
    /// let mut renderer = Renderer(String::default());
    ///
    /// valuable::visit(
    ///     &Renderable::Display(&my_struct),
    ///     &mut renderer);
    /// assert_eq!(renderer.0, "[Foo: 123, Bar: 456]");
    /// ```
    Display(
        /// The actual type to be rendered with [`Display::fmt`]
        &'a dyn Display,
    ),
}

impl<'a> Debug for Renderable<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Renderable::Debug(inner) => {
                if f.alternate() {
                    write!(f, "{:#?}", inner)
                } else {
                    write!(f, "{:?}", inner)
                }
            }
            Renderable::Display(inner) => {
                if f.alternate() {
                    write!(f, "{:#}", inner)
                } else {
                    write!(f, "{}", inner)
                }
            }
        }
    }
}

impl<'a> Renderable<'a> {
    /// Render this [`Renderable`] to the given [`Write`] target
    /// ```
    /// use valuable::{Valuable, Value, Visit, Renderable};
    /// use core::fmt;
    ///
    /// #[derive(Debug)]
    /// struct NotValuable {
    ///     foo: u32,
    ///     bar: u32,
    /// }
    ///
    /// let my_struct = NotValuable {
    ///     foo: 123,
    ///     bar: 456,
    /// };
    ///
    /// let mut buf = String::new();
    /// Renderable::Debug(&my_struct).render(&mut buf);
    /// assert_eq!(buf, "NotValuable { foo: 123, bar: 456 }");
    /// ```
    #[inline]
    pub fn render(&self, target: &mut dyn Write) -> fmt::Result {
        write!(target, "{self:?}")
    }

    /// Render this [`Renderable`] to the given [`Write`] target, but force the
    /// prettiness/alternate to be the given value
    /// ```
    /// use valuable::{Valuable, Value, Visit, Renderable};
    /// use core::fmt;
    ///
    /// #[derive(Debug)]
    /// struct NotValuable {
    ///     foo: u32,
    ///     bar: u32,
    /// }
    ///
    /// let my_struct = NotValuable {
    ///     foo: 123,
    ///     bar: 456,
    /// };
    ///
    /// let mut buf = String::new();
    /// Renderable::Debug(&my_struct).render_with_prettiness(&mut buf, true);
    /// assert_eq!(buf,
    /// "NotValuable {
    ///     foo: 123,
    ///     bar: 456,
    /// }");
    /// ```
    #[inline]
    pub fn render_with_prettiness(&self, target: &mut dyn Write, pretty: bool) -> fmt::Result {
        if pretty {
            write!(target, "{self:#?}")
        } else {
            write!(target, "{self:?}")
        }
    }

    /// Render this [`Renderable`] to an owned [`String`]
    /// ```
    /// use valuable::{Valuable, Value, Visit, Renderable};
    /// use core::fmt;
    ///
    /// #[derive(Debug)]
    /// struct NotValuable {
    ///     foo: u32,
    ///     bar: u32,
    /// }
    ///
    /// let my_struct = NotValuable {
    ///     foo: 123,
    ///     bar: 456,
    /// };
    ///
    /// let rendered = Renderable::Debug(&my_struct).render_to_string();
    /// assert_eq!(rendered, "NotValuable { foo: 123, bar: 456 }");
    /// ```
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn render_to_string(&self) -> alloc::string::String {
        format!("{self:?}")
    }

    /// Render this [`Renderable`] to an owned [`String`], but force the
    /// prettiness/alternate to be the given value
    /// ```
    /// use valuable::{Valuable, Value, Visit, Renderable};
    /// use core::fmt;
    ///
    /// #[derive(Debug)]
    /// struct NotValuable {
    ///     foo: u32,
    ///     bar: u32,
    /// }
    ///
    /// let my_struct = NotValuable {
    ///     foo: 123,
    ///     bar: 456,
    /// };
    ///
    /// let rendered = Renderable::Debug(&my_struct)
    ///     .render_to_string_with_prettiness(true);
    /// assert_eq!(rendered,
    /// "NotValuable {
    ///     foo: 123,
    ///     bar: 456,
    /// }");
    /// ```
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn render_to_string_with_prettiness(&self, pretty: bool) -> alloc::string::String {
        if pretty {
            format!("{self:#?}")
        } else {
            format!("{self:?}")
        }
    }
}

impl<'a> Valuable for Renderable<'a> {
    fn as_value(&self) -> crate::Value<'_> {
        Value::Renderable(*self)
    }

    fn visit(&self, visit: &mut dyn crate::Visit) {
        visit.visit_value(self.as_value());
    }

    fn visit_slice(slice: &[Self], visit: &mut dyn crate::Visit)
    where
        Self: Sized,
    {
        visit.visit_primitive_slice(Slice::Renderable(slice));
    }
}
