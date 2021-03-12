use crate::Value;

pub trait Valuable {
    fn as_value(&self) -> Value<'_>;
}
