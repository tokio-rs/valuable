use crate::{Fields, Field, Value};

use std::fmt;

pub trait Valuable {
    fn fields(&self) -> Fields;

    // fn field_by_name(&self, name: &str) -> Option<Field>;

    fn get(&self, field: &Field) -> Option<Value<'_>>;
}

pub(crate) fn debug(valuable: &dyn Valuable, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut f = fmt.debug_struct("");

    for field in valuable.fields().iter() {
        f.field(field.name(), &valuable.get(&field).unwrap());
    }

    f.finish()
}

/*
use std::any::TypeId;

pub struct Fields {
    type_id: TypeId,
    definitions: &'static [FieldDefinition]
}

pub struct Field {
    type_id: TypeId,
    definition: &'static FieldDefinition,
}

/// Field definition
struct FieldDefinition {
    name: &'static str,
    ty: Type,
}

enum Type {
    u8,
    u16,
    u32,
    u64,
    String,
    // ... more here
    Valuable,
    Mappable,
    Listable,
}

struct Value<'a> {

}

enum Kind<'a> {
    U8(u8),
    // ... more here
    Valuable(&'a dyn Valuable),
    Mappable(&'a dyn Mappable),
    Listable(&'a dyn Listable),
}

pub trait Valuable {
    fn fields(&self) -> Fields;

    fn field_by_name(&self, name: &str) -> Option<Field>;

    fn get(&self, field: &Field) -> Option<Value<'_>>;
}

pub trait Listable {
    fn get(&self, index: usize) -> Option<Value<'_>>;
}

pub trait Mappable {
    fn get(&self, key: &Value<'_>) -> Option<Value<'_>>;
}

struct HelloWorld {
    hello: &'static str,
    world: u16,
}

static HELLO_WORLD_FIELDS: &'static [FieldDefinition] => &[
    FieldDefinition { name: "hello", ty: Type::String },
    FieldDefinition { name: "world", ty: Type::U16 },
];

impl Valuable for HelloWorld {
    fn fields(&self) -> Fields {
        Fields {
            ty: TypeId::of::<Self>::(),
            definitions: HELLO_WORLD_FIELDS,
        }
    }

    fn field_by_name(&self, name: &str) -> Option<Field> {
        unimplemented!();
    }

    fn get(&self, field: &Field) -> Option<Value<'_>> {
        // This is a bit fuzzy
        match field {
            HELLO_WORLD_FIELDS[0] => Some(Value::string(&self.hello)),
            HELLO_WORLD_FIELDS[1] => Some(Value::from_u16(&self.world)),
            _ => None,
        }
    }
}
*/