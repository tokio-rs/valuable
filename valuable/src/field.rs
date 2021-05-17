#[derive(Debug)]
pub enum Fields<'a> {
    /// Named fields
    Named(&'a [NamedField<'a>]),

    /// Unnamed (positional) fields or unit
    Unnamed,
}

#[derive(Debug)]
pub struct NamedField<'a>(&'a str);

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
        NamedField(name)
    }

    pub fn name(&self) -> &str {
        self.0
    }
}
