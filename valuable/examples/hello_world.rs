use valuable::*;
use valuable::field::*;

struct HelloWorld {
    hello: &'static str,
    world: World,
}

struct World {
    answer: usize,
}

static HELLO_WORLD_FIELDS: &[NamedField<'static>] =
    &[NamedField::new("hello"), NamedField::new("world")];

impl Structable for HelloWorld {
    fn definition(&self) -> StructDef<'_> {
        StructDef {
            name: "HelloWorld",
            static_fields: HELLO_WORLD_FIELDS,
            is_dynamic: false,
        }
    }

    fn visit(&self, v: &mut dyn Visit) {
        v.visit_named_fields(&NamedValues::new(
            HELLO_WORLD_FIELDS,
            &[Value::String(self.hello), Value::Structable(&self.world)],
        ));
    }
}

static WORLD_FIELDS: &'static [NamedField<'static>] = &[NamedField::new("answer")];

impl Structable for World {
    fn definition(&self) -> StructDef<'_> {
        StructDef {
            name: "World",
            static_fields: WORLD_FIELDS,
            is_dynamic: false,
        }
    }

    fn visit(&self, v: &mut dyn Visit) {
        v.visit_named_fields(&NamedValues::new(WORLD_FIELDS, &[Value::Usize(self.answer)]));
    }
}

fn main() {
    let hello_world = HelloWorld {
        hello: "wut",
        world: World { answer: 42 },
    };

    let value = Value::Structable(&hello_world);
    println!("{:#?}", value);
}
