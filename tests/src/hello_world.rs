use valuable::*;
use valuable::field::*;

#[derive(Default, Debug)]
pub struct HelloWorld {
    pub id: u32,
}

static FIELDS: &[NamedField<'static>] = &[NamedField::new("id")];

impl Valuable for HelloWorld {
    fn as_value(&self) -> Value<'_> {
        Value::Structable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        visit.visit_named_fields(&NamedValues::new(FIELDS, &[Value::U32(self.id)]));
    }
}

impl Structable for HelloWorld {
    fn definition(&self) -> StructDef<'_> {
        StructDef {
            name: "HelloWorld",
            fields: Fields::NamedStatic(FIELDS),
            is_dynamic: false,
        }
    }
}