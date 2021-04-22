
use valuable::*;

struct HelloWorld {
    hello: &'static str,
    world: World,
}

struct World {
    answer: usize,
}

static HELLO_WORLD_FIELDS: &[StaticField] = &[
    StaticField::new("hello"),
    StaticField::new("world"),
];

type Iter<'a> = &'a mut dyn Iterator<Item = (Field<'a>, Value<'a>)>;

impl Structable for HelloWorld {
    fn definition(&self) -> Definition<'_> {
        Definition {
            name: "HelloWorld",
            static_fields: HELLO_WORLD_FIELDS,
            is_dynamic: false,
        }
    }

    fn get(&self, field: &Field<'_>) -> Option<Value<'_>> {
        match field {
            Field::Static(field) => {
                if *field == &HELLO_WORLD_FIELDS[0] {
                    Some(Value::String(self.hello))
                } else if *field == &HELLO_WORLD_FIELDS[1] {
                    Some(Value::Structable(&self.world))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn with_iter_fn_mut(&self, f: &mut dyn FnMut(Iter<'_>)) {
        f(&mut [
            (Field::Static(&HELLO_WORLD_FIELDS[0]), Value::String(self.hello)),
            (Field::Static(&HELLO_WORLD_FIELDS[1]), Value::Structable(&self.world))
        ].iter().map(|(f, v)| (*f, v.as_value())));
    }
}

static WORLD_FIELDS: &'static [StaticField] = &[
    StaticField::new("answer"),
];

impl Structable for World {
    fn definition(&self) -> Definition<'_> {
        Definition {
            name: "World",
            static_fields: WORLD_FIELDS,
            is_dynamic: false,
        }
    }

    fn get(&self, field: &Field<'_>) -> Option<Value<'_>> {
        match field {
            Field::Static(field) => {
                if *field == &WORLD_FIELDS[0] {
                    Some(Value::Usize(self.answer))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn with_iter_fn_mut(&self, f: &mut dyn FnMut(Iter<'_>)) {
        f(&mut [
            (Field::Static(&WORLD_FIELDS[0]), Value::Usize(self.answer)),
        ].iter().map(|(f, v)| (*f, v.as_value())));
    }
}

fn main() {
    let hello_world = HelloWorld {
        hello: "wut",
        world: World {
            answer: 42,
        },
    };

    let value = Value::Structable(&hello_world);
    println!("{:#?}", value);
}