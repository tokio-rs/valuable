
/// A struct or enum fields.
pub struct FieldsDef<'a> {
    /// Named or unnamed fields
    fields: Fields<'a>,

    /// True if not all fields are statically known
    is_dynamic: bool,
}

pub enum Fields<'a> {
    /// Named fields
    Named(&'a [NamedField<'a>]),

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
