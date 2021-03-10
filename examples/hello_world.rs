
use valuable::{Field, FieldDefinition, Fields, Type, Valuable, Value};

use core::any::TypeId;

struct HelloWorld {
    hello: &'static str,
    world: usize,
}

static FIELDS: &'static [FieldDefinition] = &[
    FieldDefinition { name: "hello", ty: Type::String },
    FieldDefinition { name: "world", ty: Type::Usize },
];

impl Valuable for HelloWorld {
    fn fields(&self) -> Fields {
        Fields {
            type_id: TypeId::of::<Self>(),
            definitions: FIELDS,
        }
    }

    fn get(&self, field: &Field) -> Option<Value<'_>> {
        if field.type_id != TypeId::of::<Self>() {
            None
        } else if field.definition == &FIELDS[0] {
            Some(Value::String(&self.hello))
        } else if field.definition == &FIELDS[1] {
            Some(Value::Usize(self.world))
        } else {
            None
        }
    }
}

fn main() {
    let hello_world = HelloWorld {
        hello: "wut",
        world: 42,
    };

    let value = Value::Valuable(&hello_world);
    println!("{:?}", value);
}