#[cfg(feature = "alloc")]
use alloc::{string::String, sync::Arc};
use core::fmt;

/// Data stored within a `Structable` or  an `Enumerable`.
#[derive(Debug)]
pub enum Fields<'a> {
    /// Named fields
    Named(&'a [NamedField<'a>]),

    /// Unnamed (positional) fields or unit
    Unnamed,
}

/// A named field
#[derive(Clone)]
pub struct NamedField<'a>(NamedFieldInner<'a>);

#[derive(Clone)]
enum NamedFieldInner<'a> {
    Borrowed(&'a str),
    #[cfg(feature = "alloc")]
    Owned(Arc<str>),
}

impl Fields<'_> {
    /// Returns `true` if the fields are named.
    ///
    /// # Examples
    ///
    /// Named fields
    ///
    /// ```
    /// use valuable::Fields;
    ///
    /// let fields = Fields::Named(&[]);
    /// assert!(fields.is_named());
    /// ```
    ///
    /// Unnamed fields
    ///
    /// ```
    /// use valuable::Fields;
    ///
    /// let fields = Fields::Unnamed;
    /// assert!(!fields.is_named());
    /// ```
    pub fn is_named(&self) -> bool {
        matches!(self, Fields::Named(..))
    }

    /// Returns `true` if the fields are unnamed.
    ///
    /// # Examples
    ///
    /// Named fields
    ///
    /// ```
    /// use valuable::Fields;
    ///
    /// let fields = Fields::Named(&[]);
    /// assert!(!fields.is_unnamed());
    /// ```
    ///
    /// Unnamed fields
    ///
    /// ```
    /// use valuable::Fields;
    ///
    /// let fields = Fields::Unnamed;
    /// assert!(fields.is_unnamed());
    /// ```
    pub fn is_unnamed(&self) -> bool {
        matches!(self, Fields::Unnamed)
    }
}

impl<'a> NamedField<'a> {
    /// Create a new `NamedField` instance with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::NamedField;
    ///
    /// let field = NamedField::new("hello");
    /// assert_eq!("hello", field.name());
    /// ```
    pub const fn new(name: &'a str) -> NamedField<'a> {
        NamedField(NamedFieldInner::Borrowed(name))
    }

    /// Create a new `NamedField` instance from an owned [`String`].
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::NamedField;
    ///
    /// let what = "world";
    /// let name = format!("hello_{}", what);
    /// let field = NamedField::from_string(name);
    /// assert_eq!("hello_world", field.name());
    /// ```
    #[cfg(feature = "alloc")]
    pub fn from_string(name: String) -> NamedField<'static> {
        NamedField(NamedFieldInner::Owned(Arc::from(name)))
    }

    /// Returns the field name
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::NamedField;
    ///
    /// let field = NamedField::new("hello");
    /// assert_eq!("hello", field.name());
    /// ```
    pub fn name(&self) -> &str {
        match self.0 {
            NamedFieldInner::Borrowed(name) => name,
            #[cfg(feature = "alloc")]
            NamedFieldInner::Owned(ref name) => name,
        }
    }
}

impl<'a> fmt::Debug for NamedField<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NamedField").field(&self.name()).finish()
    }
}

impl<'a> From<&'a str> for NamedField<'a> {
    fn from(name: &'a str) -> Self {
        Self::new(name)
    }
}

#[cfg(feature = "alloc")]
impl From<String> for NamedField<'static> {
    fn from(name: String) -> Self {
        Self::from_string(name)
    }
}

#[cfg(feature = "alloc")]
impl From<Arc<str>> for NamedField<'static> {
    fn from(name: Arc<str>) -> Self {
        Self(NamedFieldInner::Owned(name))
    }
}
