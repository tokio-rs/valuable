pub enum Fields<'a> {
    /// Named fields
    Named(&'a [NamedField<'a>]),

    /// Static named fields
    NamedStatic(&'static [NamedField<'static>]),

    /// Unnamed (positional) fields or unit
    Unnamed,
}

pub struct NamedField<'a> {
    /// Field name
    name: &'a str,
}

impl Fields<'_> {
    pub fn is_named(&self) -> bool {
        match self {
            Fields::Named(..) | Fields::NamedStatic(..) => true,
            _ => false,
        }
    }

    pub fn is_unnamed(&self) -> bool {
        match self {
            Fields::Unnamed => true,
            _ => false,
        }
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
