use crate::{Definition, Field, StaticField, Value};

/// Access values for a struct's static fields
pub struct Record<'a> {
    definition: &'a Definition<'a>,
    values: &'a [Value<'a>],
}

impl<'a> Record<'a> {
    pub fn new(definition: &'a Definition<'a>, values: &'a [Value<'a>]) -> Record<'a> {
        Record { definition, values }
    }

    pub fn definition(&self) -> &Definition<'_> {
        self.definition
    }

    /// TODO: micro optimizations
    pub fn get_static_unchecked(&self, field: &'static StaticField) -> &Value<'_> {
        &self.values[field.index()]
    }

    pub fn get(&self, field: &Field<'_>) -> Option<&Value<'_>> {
        match *field {
            Field::Static(f) => {
                let i = f.index();
                assert!(
                    self.definition.is_member(f),
                    "field member of different struct"
                );

                self.values.get(i)
            }
            _ => unimplemented!(),
        }
    }

    pub fn entries<'b>(&'b self) -> impl Iterator<Item = (&'static StaticField, &Value<'a>)> + 'b {
        self.definition
            .static_fields()
            .iter()
            .map(move |field| (field, &self.values[field.index()]))
    }
}
