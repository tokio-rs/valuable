use valuable::field::*;
use valuable::*;

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
        StructDef::new("HelloWorld", Fields::NamedStatic(HELLO_WORLD_FIELDS), false)
    }
}

impl Valuable for HelloWorld {
    fn as_value(&self) -> Value<'_> {
        Value::Structable(self)
    }

    fn visit(&self, v: &mut dyn Visit) {
        v.visit_named_fields(&NamedValues::new(
            HELLO_WORLD_FIELDS,
            &[Value::String(self.hello), Value::Structable(&self.world)],
        ));
    }
}

static WORLD_FIELDS: &'static [NamedField<'static>] = &[NamedField::new("answer")];

impl Valuable for World {
    fn as_value(&self) -> Value<'_> {
        Value::Structable(self)
    }

    fn visit(&self, v: &mut dyn Visit) {
        v.visit_named_fields(&NamedValues::new(
            WORLD_FIELDS,
            &[Value::Usize(self.answer)],
        ));
    }
}

impl Structable for World {
    fn definition(&self) -> StructDef<'_> {
        StructDef::new("World", Fields::NamedStatic(WORLD_FIELDS), false)
    }
}

fn main() {
    let hello_world = HelloWorld {
        hello: "wut",
        world: World { answer: 42 },
    };

    let value = Value::Structable(&hello_world);
    println!("{:#?}", value);

    let slice = &[1, 2, 3][..];

    let value = &slice as &dyn Valuable;
    println!("{:?}", value);
}
