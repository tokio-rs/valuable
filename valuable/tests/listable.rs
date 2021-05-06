use valuable::field::*;
use valuable::*;

#[derive(Default, Debug)]
struct HelloWorld {
    id: u32,
}

static FIELDS: &[NamedField<'static>] = &[NamedField::new("num")];

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

#[test]
#[ignore]
fn test_default_visit_slice_empty() {
    // let empty: Vec<HelloWorld> = vec![];

    unimplemented!()
}

#[test]
#[ignore]
fn test_default_visit_slice_small() {
    unimplemented!()
}

#[test]
#[ignore]
fn test_default_visit_slice_big() {
    unimplemented!()
}

#[test]
#[ignore]
fn test_visit_slice_primitive() {
    unimplemented!()
}
