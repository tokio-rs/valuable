
use valuable::{Field, FieldDefinition, Fields, Type, Valuable, Value};

use core::any::TypeId;

struct HelloWorld {
    hello: &'static str,
    world: World,
}

struct World {
    answer: usize,
}

static HELLO_WORLD_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition { name: "hello", ty: Type::String },
    FieldDefinition { name: "world", ty: Type::Valuable },
];

impl Valuable for HelloWorld {
    fn fields(&self) -> Fields {
        Fields {
            type_id: TypeId::of::<Self>(),
            definitions: HELLO_WORLD_FIELDS,
        }
    }

    fn get(&self, field: &Field) -> Option<Value<'_>> {
        if field.type_id != TypeId::of::<Self>() {
            None
        } else if field.definition == &HELLO_WORLD_FIELDS[0] {
            Some(Value::String(&self.hello))
        } else if field.definition == &HELLO_WORLD_FIELDS[1] {
            Some(Value::Valuable(&self.world))  
        } else {
            None
        }
    }
}

static WORLD_FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition { name: "answer", ty: Type::Usize },
];

impl Valuable for World {
    fn fields(&self) -> Fields {
        Fields {
            type_id: TypeId::of::<Self>(),
            definitions: WORLD_FIELDS,
        }
    }

    fn get(&self, field: &Field) -> Option<Value<'_>> {
        if field.type_id != TypeId::of::<Self>() {
            None
        } else if field.definition == &WORLD_FIELDS[0] {
            Some(Value::Usize(self.answer))
        } else {
            None
        }
    }
}

fn main() {
    let hello_world = HelloWorld {
        hello: "wut",
        world: World {
            answer: 42,
        },
    };

    let value = Value::Valuable(&hello_world);
    println!("{:?}", value);
}