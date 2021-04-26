use crate::*;
use crate::field::*;

pub trait Enumerable {
    fn visit(&self, visitor: &mut dyn Visit);
}

pub struct EnumDef<'a> {
    /// Enum type name
    name: &'a str,

    /// Known variants
    variants: &'a [VariantDef<'a>],

    /// `true` when not all variants are statically known
    is_dynamic: bool,
}

pub struct VariantDef<'a> {
    /// Variant name
    name: &'a str,

    /// Static named fields
    pub static_fields: &'static [NamedField<'static>],

}