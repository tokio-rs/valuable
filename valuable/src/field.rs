/// Data stored within a `Structable` or  an `Enumerable`.
#[derive(Debug)]
pub enum Fields<'a> {
    /// Named fields
    Named(&'a [NamedField<'a>]),

    /// Unnamed (positional) fields or unit
    Unnamed,
}

/// A named field
#[derive(Debug)]
pub struct NamedField<'a>(&'a str);

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
    pub fn name(&self) -> &str {
        self.0
    }
}
