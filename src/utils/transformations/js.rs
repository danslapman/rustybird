use serde_json::Value;

pub trait JsonTransformations {
    fn transform_values(&mut self, modify: fn(Value) -> Value) -> Self;
}

impl JsonTransformations for &Value {
    fn transform_values(&mut self, modify: fn(Value) -> Value) -> Self {
        todo!()
    }
}