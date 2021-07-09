use serde_json::{Value as JsonValue};
use valuable::{Valuable, Value, Visit};

struct Json<'a>(&'a JsonValue);

impl<'a> Valuable for Json<'a> {
    fn as_value(&self) -> Value<'a> {
        match self.0 {
            // TODO: fixme
            JsonValue::Array(ref array) => Value::Listable(array),
            JsonValue::Bool(ref value) => Value::Bool(*value),
            JsonValue::Number(ref num) => {
                // TODO: check correctness for this
                if num.is_f64() {
                    Value::F64(num.as_f64().unwrap())
                } else if num.is_i64() {
                    Value::I64(num.as_i64().unwrap())
                } else if num.is_u64() {
                    Value::U64(num.as_u64().unwrap())
                } else {
                    unreachable!()
                }
            }
            JsonValue::Null => Value::Unit,
            JsonValue::String(ref s) => Value::String(s),
            JsonValue::Object(ref object) => {
                // TODO: make map valuable
                Value::Mappable(object),
            }
        }
    }

    fn visit(&self, visit: &mut dyn Visit) {
        //TODO:
    }
}
