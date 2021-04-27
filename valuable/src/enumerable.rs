use crate::field::*;
use crate::*;

use core::fmt;

pub trait Enumerable {
    fn definition(&self) -> EnumDef<'_>;

    fn visit(&self, visitor: &mut dyn Visit);
}

pub struct EnumDef<'a> {
    /// Enum type name
    pub name: &'a str,

    /// Known variants
    pub variants: &'a [VariantDef<'a>],

    /// `true` when not all variants are statically known
    pub is_dynamic: bool,
}

pub struct VariantDef<'a> {
    /// Variant name
    pub name: &'a str,

    pub fields: Fields<'a>,

    /// Are all fields statically known?
    pub is_dynamic: bool,
}

pub struct Variant<'a> {
    name: &'a str,
}

impl EnumDef<'_> {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn variants(&self) -> &[VariantDef<'_>] {
        self.variants
    }

    pub fn is_dynamic(&self) -> bool {
        self.is_dynamic
    }
}

impl Variant<'_> {
    pub fn name(&self) -> &str {
        self.name
    }
}

impl fmt::Debug for dyn Enumerable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DebugEnumerable<'a, 'b> {
            name: &'b str,
            fmt: &'b mut fmt::Formatter<'a>,
            res: fmt::Result,
        };

        impl Visit for DebugEnumerable<'_, '_> {
            fn visit_variant_named_fields(
                &mut self,
                variant: &Variant<'_>,
                named_values: &NamedValues<'_>,
            ) {
                let name = format!("{}::{}", self.name, variant.name());
                let mut d = self.fmt.debug_struct(&name);

                for (field, value) in named_values.entries() {
                    d.field(field.name(), value);
                }

                self.res = d.finish();
            }

            fn visit_variant_unnamed_fields(
                &mut self,
                variant: &Variant<'_>,
                values: &[Value<'_>],
            ) {
                let name = format!("{}::{}", self.name, variant.name());
                let mut d = self.fmt.debug_tuple(&name);

                for value in values {
                    d.field(value);
                }

                self.res = d.finish();
            }
        }

        let def = self.definition();
        let mut visit = DebugEnumerable {
            name: def.name(),
            fmt,
            res: Ok(()),
        };

        self.visit(&mut visit);
        visit.res
    }
}
