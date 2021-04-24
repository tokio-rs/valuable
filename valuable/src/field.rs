

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum Field<'a> {
    Static(&'static StaticField),
    Dynamic(DynamicField<'a>),
}

pub struct StaticField {
    /// Index in the struct's record
    index: usize,

    /// Field name
    name: &'static str,
}

#[derive(Clone, Copy)]
pub struct DynamicField<'a> {
    name: &'a str,
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
    pub const fn new(index: usize, name: &'static str) -> StaticField {
        StaticField { index, name: name }
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

impl DynamicField<'_> {
    pub fn name(&self) -> &str {
        self.name
    }
}
