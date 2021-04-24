use valuable::*;
use valuable::field::*;

struct HelloWorld {
    hello: &'static str,
    world: World,
}

struct World {
    answer: usize,
}

static HELLO_WORLD_FIELDS: &[StaticField] =
    &[StaticField::new(0, "hello"), StaticField::new(1, "world")];

impl Structable for HelloWorld {
    fn definition(&self) -> StructDef<'_> {
        StructDef {
            name: "HelloWorld",
            static_fields: HELLO_WORLD_FIELDS,
            is_dynamic: false,
        }
    }

    fn visit(&self, v: &mut dyn Visit) {
        let definition = self.definition();
        v.visit_named_fields(&NamedValues::new(
            &definition,
            &[Value::String(self.hello), Value::Structable(&self.world)],
        ));
    }
}

static WORLD_FIELDS: &'static [StaticField] = &[StaticField::new(0, "answer")];

impl Structable for World {
    fn definition(&self) -> StructDef<'_> {
        StructDef {
            name: "World",
            static_fields: WORLD_FIELDS,
            is_dynamic: false,
        }
    }

    fn visit(&self, v: &mut dyn Visit) {
        let definition = self.definition();
        v.visit_named_fields(&NamedValues::new(&definition, &[Value::Usize(self.answer)]));
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
