pub enum Fields<'a> {
    /// Named fields
    Named(&'a [NamedField<'a>]),

    /// Unnamed (positional) fields or unit
    Unnamed,
}

pub struct NamedField<'a> {
    /// Field name
    name: &'a str,
}

impl Fields<'_> {
    pub fn is_named(&self) -> bool {
        matches!(self, Fields::Named(..))
    }

    pub fn is_unnamed(&self) -> bool {
        matches!(self, Fields::Unnamed)
    }
}

impl<'a> NamedField<'a> {
    pub const fn new(name: &'a str) -> NamedField<'a> {
        NamedField { name: name }
    }

    pub fn name(&self) -> &str {
        self.name
    }
}
