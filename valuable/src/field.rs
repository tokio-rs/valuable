use core::any::TypeId;
use core::cmp::PartialEq;

// TODO: Don't make all these fields public

pub struct Definition<'a> {
    /// Type name
    pub name: &'a str,

    /// Static fields
    pub static_fields: &'static [StaticField],

    /// If not all fields are statically known, then true
    pub is_dynamic: bool,
}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum Field<'a> {
    Static(&'static StaticField),
    Dynamic(DynamicField<'a>),
}

pub struct StaticField {
    name: &'static str,
}

#[derive(Clone, Copy)]
pub struct DynamicField<'a> {
    name: &'a str,
}

impl Definition<'_> {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn static_fields(&self) -> &'static [StaticField] {
        self.static_fields
    }
}

impl Field<'_> {
    pub fn name(&self) -> &str {
        match self {
            Field::Static(f) => f.name(),
            Field::Dynamic(f) => f.name(),
        }
    }
}

impl StaticField {
    pub const fn new(name: &'static str) -> StaticField {
        StaticField {
            name,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

impl PartialEq for &'static StaticField {
    fn eq(&self, other: &&'static StaticField) -> bool {
        core::ptr::eq(*self, *other)
    }
}

impl Eq for &'static StaticField {}

impl DynamicField<'_> {
    pub fn name(&self) -> &str {
        self.name
    }
}