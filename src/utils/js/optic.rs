use std::fmt::{Display, Formatter};
use serde_json::json;
use serde_json::Value;

#[derive(Clone)]
pub enum PathPart {
    Field(String),
    Index(usize),
    Traverse,
}

impl Display for PathPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathPart::Field(str) => write!(f, "{}", str),
            PathPart::Index(idx) => write!(f, "[{}]", idx),
            PathPart::Traverse => write!(f, "$")
        }
    }
}

#[derive(Clone)]
pub struct JsonOptic {
    json_path: Vec<PathPart>,
}

impl JsonOptic {
    pub fn empty() -> JsonOptic {
        JsonOptic { json_path: vec![] }
    }

    pub fn field(mut self, rhs: String) -> JsonOptic {
        self.json_path.push(PathPart::Field(rhs));
        self
    }

    pub fn index(mut self, rhs: usize) -> JsonOptic {
        self.json_path.push(PathPart::Index(rhs));
        self
    }

    pub fn traverse(mut self) -> JsonOptic {
        self.json_path.push(PathPart::Traverse);
        self
    }
}

impl Display for JsonOptic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@->{}", self.json_path.iter().map(|part| part.to_string()).collect::<Vec<_>>().join("."))
    }
}

fn construct(part: &PathPart, value: Value) -> Value {
    match part {
        PathPart::Field(name) => json!({ name: value }),
        PathPart::Index(idx) => {
            let mut blank = vec![Value::Null; *idx];
            blank.push(value);
            Value::Array(blank)
        }
        PathPart::Traverse => json!([value]),
    }
}

trait ValueExt {
    fn set(&mut self, optic: &JsonOptic, v: &Value);
}

impl ValueExt for Value {
    fn set(&mut self, optic: &JsonOptic, v: &Value) {
        let init: Box<dyn Fn(&mut Value)> = Box::new(|arg: &mut Value| {*arg = v.clone()});

        let modify_fn = optic.json_path.iter().rfold(init, |acc, el| {
            Box::new(move |arg: &mut Value| {
                arg.modify_part_in_place(el, |v_ref| { acc(v_ref); ()}, || Value::Null);
            })
        });

        let _ = modify_fn(self);
    }
}

trait ValueExtInternal {
    fn modify_part_in_place(&mut self, part: &PathPart, modify: impl Fn(&mut Value) -> (), default: impl Fn() -> Value);
    fn verify(&self, part: &PathPart) -> bool;
}

impl ValueExtInternal for Value {
    fn modify_part_in_place(&mut self, part: &PathPart, modify: impl Fn(&mut Value) -> (), default: impl Fn() -> Value) {
        match part {
            PathPart::Field(name) => match self {
                Value::Object(jo) => {
                    if let Some(jv) = jo.get_mut(name) {
                        modify(jv);
                    } else {
                        let mut new_val = default();
                        modify(&mut new_val);
                        jo.insert(name.clone(), new_val);
                    }
                }
                _ => {
                    let mut new_val = default();
                    modify(&mut new_val);
                    *self = construct(&part, new_val)
                }
            },
            PathPart::Index(idx) => match self {
                Value::Array(ja) => {
                    if ja.len() <= *idx {
                        let items_to_add = *idx - ja.len() + 1;
                        ja.append(&mut vec![Value::Null; items_to_add]);

                    }

                    modify(ja.get_mut(*idx).unwrap())
                },
                _ => {
                    let mut new_val = default();
                    modify(&mut new_val);
                    *self = construct(&part, new_val)
                }
            },
            PathPart::Traverse => match self {
                Value::Array(ja) => for jv in ja.iter_mut() {
                    modify(jv);
                },
                _ => {
                    let mut new_val = default();
                    modify(&mut new_val);
                    *self = construct(&part, new_val)
                }
            },
        }
    }

    fn verify(&self, part: &PathPart) -> bool {
        match part {
            PathPart::Field(name) => self.as_object().map(|m| m.contains_key(name)).unwrap_or(false),
            PathPart::Index(idx) => self.as_array().map(|a| a.len() > *idx).unwrap_or(false),
            PathPart::Traverse => self.is_array()
        }
    }
}

#[cfg(test)]
mod optic_tests {
    use serde_json::{json, Value};
    use crate::utils::js::optic::{JsonOptic, ValueExt};

    #[test]
    fn setter_should_create_fields_recursively_in_empty_json() {
        let optic = JsonOptic::empty().field("outer".to_string()).field("inner".to_string());
        let mut target: Value = json!({});

        target.set(&optic, &Value::from(42));

        assert_eq!(target, json!({"outer": {"inner": 42}}))
    }

    #[test]
    fn setter_should_replace_existing_value() {
        let optic = JsonOptic::empty().field("outer".to_string()).field("inner".to_string());
        let mut target: Value = json!({"outer": {"inner": 12}});

        target.set(&optic, &Value::from(42));

        assert_eq!(target, json!({"outer": {"inner": 42}}))
    }

    #[test]
    fn setter_should_keep_target_contents() {
        let optic = JsonOptic::empty().field("outer".to_string()).field("inner".to_string());
        let mut target: Value = json!({"a": {"b": "c"}});

        target.set(&optic, &Value::from(42));

        assert_eq!(target, json!({"outer": {"inner": 42}, "a": {"b": "c"}}))
    }
}