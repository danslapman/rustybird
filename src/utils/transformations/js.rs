use serde_json::Value;

pub trait JsonTransformations {
    fn transform_values_in_place(&mut self, modify: fn(&mut Value) -> ());
}

impl JsonTransformations for Value {
    fn transform_values_in_place(&mut self, modify: fn(&mut Value) -> ()) {
        match self {
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => modify(self),
            Value::Array(vs) => vs.iter_mut().for_each(|el| el.transform_values_in_place(modify)),
            Value::Object(kvs) => kvs.iter_mut().for_each(|(_, val)| val.transform_values_in_place(modify))
        }
    }
}