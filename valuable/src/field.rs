
pub enum Fields<'a> {
    /// Named fields
    Named(&'a [NamedField<'a>]),

    /// Static named fields
    NamedStatic(&'static [NamedField<'static>]),

    /// Unnamed (positional) fields
    Unnamed,

    /// No fields
    Unit,
}

pub struct NamedField<'a> {
    /// Field name
    name: &'a str,
}

impl<'a> NamedField<'a> {
    pub const fn new(name: &'a str) -> NamedField<'a> {
        NamedField { name: name }
    }

    pub fn name(&self) -> &str {
        self.name
    }
}
