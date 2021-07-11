use serde_json::{Map, Value as Json};

use crate::{Mappable, Valuable, Value, Visit};

impl Valuable for Json {
    fn as_value(&self) -> Value<'_> {
        match self {
            Json::Array(ref array) => array.as_value(),
            Json::Bool(ref value) => value.as_value(),
            Json::Number(ref num) => {
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
            Json::Null => Value::Unit,
            Json::String(ref s) => s.as_value(),
            Json::Object(ref object) => object.as_value(),
        }
    }

    fn visit(&self, visit: &mut dyn Visit) {
        match self {
            Json::Array(ref array) => array.visit(visit),
            Json::Bool(ref value) => value.visit(visit),
            Json::Number(ref num) => {
                // TODO: check correctness for this
                if num.is_f64() {
                    num.as_f64().unwrap().visit(visit)
                } else if num.is_i64() {
                    num.as_i64().unwrap().visit(visit)
                } else if num.is_u64() {
                    num.as_u64().unwrap().visit(visit)
                } else {
                    unreachable!()
                }
            }
            Json::Null => Value::Unit.visit(visit),
            Json::String(ref s) => s.visit(visit),
            Json::Object(ref object) => object.visit(visit),
        }
    }
}

impl Valuable for Map<String, Json> {
    fn as_value(&self) -> Value<'_> {
        Value::Mappable(self)
    }

    fn visit(&self, visit: &mut dyn Visit) {
        for (k, v) in self.iter() {
            visit.visit_entry(k.as_value(), v.as_value());
        }
    }
}

impl Mappable for Map<String, Json> {
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
