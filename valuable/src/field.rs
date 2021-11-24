use core::{fmt, ops::Index, slice};

/// Data stored within a `Structable` or  an `Enumerable`.
#[derive(Debug)]
pub enum Fields<'a> {
    /// Named fields
    Named(&'a Names<'a>),

    /// Unnamed (positional) fields or unit
    Unnamed,
}

/// A named field
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct NamedField<'a>(&'a str);

pub struct Names<'a>(NamesInner<'a>);

enum NamesInner<'a> {
    Borrowed(&'a [NamedField<'a>]),
    #[cfg(feature = "alloc")]
    Owned(alloc::boxed::Box<[NamedField<'a>]>),
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
        NamedField(name)
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
    pub fn name(&self) -> &'a str {
        self.0
    }
}

impl<'a> Names<'a> {
    pub const fn new(names: &'a [NamedField<'a>]) -> Self {
        Self(NamesInner::Borrowed(names))
    }

    #[cfg(feature = "alloc")]
    pub const fn new_owned(names: Box<[NamedField<'a>]>) -> Self {
        Self(NamesInner::Owned(names))
    }

    pub fn names(&self) -> &[NamedField<'a>] {
        match self {
            Self(NamesInner::Borrowed(names)) => names,

            #[cfg(feature = "alloc")]
            Self(NamesInner::Owned(names)) => names.as_ref(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self(NamesInner::Borrowed(names)) => names.len(),

            #[cfg(feature = "alloc")]
            Self(NamesInner::Owned(names)) => names.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self(NamesInner::Borrowed(names)) => names.is_empty(),

            #[cfg(feature = "alloc")]
            Self(NamesInner::Owned(names)) => names.is_empty(),
        }
    }

    pub fn iter(&self) -> slice::Iter<'_, NamedField<'a>> {
        self.names().iter()
    }
}

impl fmt::Debug for Names<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.names().fmt(f)
    }
}

impl<'a> AsRef<[NamedField<'a>]> for Names<'a> {
    fn as_ref(&self) -> &[NamedField<'a>] {
        self.names()
    }
}

impl<'a, I> Index<I> for Names<'a>
where
    [NamedField<'a>]: Index<I>,
{
    type Output = <[NamedField<'a>] as Index<I>>::Output;

    #[inline]
    fn index(&self, i: I) -> &Self::Output {
        self.names().index(i)
    }
}

impl<'a, 'b> IntoIterator for &'b Names<'a> {
    type Item = &'b NamedField<'a>;
    type IntoIter = slice::Iter<'b, NamedField<'a>>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_indexes() {
        // This is really intended to test that `Index` is implemented for the
        // various different types of slice index, so it's really a compile
        // test...obviously, the indexing will return the expected values...
        let names = &[
            NamedField::new("hello"),
            NamedField::new("world"),
            NamedField::new("have"),
            NamedField::new("lots_of_fun"),
        ];
        let names = Names::new(names);
        assert_eq!(names[0].name(), "hello");
        assert_eq!(names[3].name(), "lots_of_fun");
        assert_eq!(
            &names[..],
            &[
                NamedField::new("hello"),
                NamedField::new("world"),
                NamedField::new("have"),
                NamedField::new("lots_of_fun"),
            ]
        );
        assert_eq!(
            &names[1..2],
            &[NamedField::new("world"), NamedField::new("have"),]
        );
        assert_eq!(
            &names[1..],
            &[
                NamedField::new("world"),
                NamedField::new("have"),
                NamedField::new("lots_of_fun"),
            ]
        );
    }
}
