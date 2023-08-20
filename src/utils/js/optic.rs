use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde_json::json;
use serde_json::Value;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, PartialEq, Eq, Hash)]
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
            PathPart::Traverse => write!(f, "$"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonOptic {
    json_path: Vec<PathPart>,
}

static INDEX_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[(\d+)\]").unwrap());

impl JsonOptic {
    pub fn empty() -> JsonOptic {
        JsonOptic { json_path: vec![] }
    }

    pub fn from_path(path_str: &str) -> JsonOptic {
        JsonOptic {
            json_path: path_str.split('.').map(|s| match s {
                "$" => PathPart::Traverse,
                i if INDEX_PATTERN.is_match(i) => {
                    if let Some(index_cap) = INDEX_PATTERN.captures(i) {
                        let cap1 = &index_cap[1];
                        if let Ok(idx) = cap1.parse::<usize>() {
                            PathPart::Index(idx)
                        } else {
                            PathPart::Field(i.to_string())
                        }
                    } else {
                        PathPart::Field(i.to_string())
                    }
                },
                s => PathPart::Field(s.to_string())
            }).collect::<Vec<_>>()
        }
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
        write!(
            f,
            "{}",
            self.json_path
                .iter()
                .map(|part| part.to_string())
                .collect::<Vec<_>>()
                .join(".")
        )
    }
}

impl Debug for JsonOptic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.json_path
                .iter()
                .map(|part| part.to_string())
                .collect::<Vec<_>>()
                .join(".")
        )
    }
}

impl Serialize for JsonOptic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.json_path.iter().map(|part| part.to_string()).collect::<Vec<_>>().join(".").as_str())
    }
}

impl <'de> Deserialize<'de> for JsonOptic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        String::deserialize(deserializer).map(|str| JsonOptic::from_path(str.as_str()))
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

pub trait ValueExt {
    fn set(&mut self, optic: &JsonOptic, v: &Value);
    fn set_opt(&mut self, optic: &JsonOptic, v: Option<&Value>);
    fn prune(&mut self, optic: &JsonOptic);
    fn get_all(&self, optic: &JsonOptic) -> Vec<&Value>;
    fn validate(&self, optic: &JsonOptic) -> bool;
}

impl ValueExt for Value {
    fn set(&mut self, optic: &JsonOptic, v: &Value) {
        let init: Box<dyn Fn(&mut Value)> = Box::new(|arg: &mut Value| *arg = v.clone());

        let modify_fn = optic.json_path.iter().rfold(init, |acc, el| {
            Box::new(move |arg: &mut Value| {
                arg.modify_part_in_place(
                    el,
                    |v_ref| {
                        acc(v_ref);
                        ()
                    },
                    || Value::Null,
                );
            })
        });

        let _ = modify_fn(self);
    }

    fn set_opt(&mut self, optic: &JsonOptic, v: Option<&Value>) {
        match v {
            Some(value) => self.set(optic, value),
            None => self.prune(optic),
        }
    }

    fn prune(&mut self, optic: &JsonOptic) {
        if self.validate(optic) {
            let init: Box<dyn Fn(&mut Value)> = match &optic.json_path[optic.json_path.len() - 1] {
                PathPart::Field(field_name) => Box::new(|v: &mut Value| v.remove_field(field_name)),
                PathPart::Index(index) => Box::new(|v: &mut Value| v.remove_at_index(*index)),
                PathPart::Traverse => Box::new(|v: &mut Value| {
                    if v.is_array() {
                        *v = Value::Null
                    }
                }),
            };

            let modify_fn = optic.json_path[0..(optic.json_path.len() - 1)]
                .iter()
                .rfold(init, |acc, el| {
                    Box::new(move |arg: &mut Value| {
                        arg.modify_part_in_place(
                            el,
                            |v_ref| {
                                acc(v_ref);
                                ()
                            },
                            || Value::Null,
                        )
                    })
                });

            let _ = modify_fn(self);
        }
    }

    fn get_all(&self, optic: &JsonOptic) -> Vec<&Value> {
        optic.json_path.iter().fold(vec![self], |acc, el| match el {
            PathPart::Field(name) => acc.iter().flat_map(|vx| vx.field(name)).collect::<Vec<_>>(),
            PathPart::Index(index) => acc.iter().flat_map(|vx| vx.at_index(*index)).collect::<Vec<_>>(),
            PathPart::Traverse => acc
                .iter()
                .flat_map(|vx| vx.as_array().map(|v| v.iter().collect::<Vec<_>>()).unwrap_or(vec![]))
                .collect::<Vec<_>>(),
        })
    }

    fn validate(&self, optic: &JsonOptic) -> bool {
        if optic.json_path.iter().all(|el| *el == PathPart::Traverse) {
            self.is_array()
        } else {
            !optic
                .json_path
                .iter()
                .fold(vec![self], |acc, el| {
                    acc.iter()
                        .filter(|j| j.verify(el))
                        .flat_map(|j| match el {
                            PathPart::Field(field_name) => j.field(field_name).into_iter().collect::<Vec<_>>(),
                            PathPart::Index(index) => j.at_index(*index).into_iter().collect::<Vec<_>>(),
                            PathPart::Traverse => j.as_array().map(|v| v.iter().collect::<Vec<_>>()).unwrap_or(vec![]),
                        })
                        .collect()
                })
                .is_empty()
        }
    }
}

trait ValueExtInternal {
    fn modify_part_in_place(&mut self, part: &PathPart, modify: impl Fn(&mut Value) -> (), default: impl Fn() -> Value);
    fn verify(&self, part: &PathPart) -> bool;
    fn field(&self, field_name: &String) -> Option<&Value>;
    fn at_index(&self, index: usize) -> Option<&Value>;
    fn remove_field(&mut self, field_name: &String);
    fn remove_at_index(&mut self, index: usize);
}

impl ValueExtInternal for Value {
    fn modify_part_in_place(
        &mut self,
        part: &PathPart,
        modify: impl Fn(&mut Value) -> (),
        default: impl Fn() -> Value,
    ) {
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
                }
                _ => {
                    let mut new_val = default();
                    modify(&mut new_val);
                    *self = construct(&part, new_val)
                }
            },
            PathPart::Traverse => match self {
                Value::Array(ja) => {
                    for jv in ja.iter_mut() {
                        modify(jv);
                    }
                }
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
            PathPart::Traverse => self.is_array(),
        }
    }

    fn field(&self, field_name: &String) -> Option<&Value> {
        match self {
            Value::Object(map) => map.get(field_name),
            _ => None,
        }
    }

    fn at_index(&self, index: usize) -> Option<&Value> {
        match self {
            Value::Array(vec) => vec.get(index),
            _ => None,
        }
    }

    fn remove_field(&mut self, field_name: &String) {
        if let Some(jmap) = self.as_object_mut() {
            jmap.remove(field_name);
        }
    }

    fn remove_at_index(&mut self, index: usize) {
        if let Some(jarr) = self.as_array_mut() {
            jarr.remove(index);
        }
    }
}

#[cfg(test)]
mod optic_tests {
    use crate::utils::js::optic::{JsonOptic, ValueExt};
    use serde_json::{json, Value};

    #[test]
    fn setter_should_create_fields_recursively_in_empty_json() {
        let optic = JsonOptic::from_path("outer.inner");
        let mut target: Value = json!({});

        target.set(&optic, &Value::from(42));

        assert_eq!(target, json!({"outer": {"inner": 42}}))
    }

    #[test]
    fn setter_should_replace_existing_value() {
        let optic = JsonOptic::from_path("outer.inner");
        let mut target: Value = json!({"outer": {"inner": 12}});

        target.set(&optic, &Value::from(42));

        assert_eq!(target, json!({"outer": {"inner": 42}}))
    }

    #[test]
    fn setter_should_keep_target_contents() {
        let optic = JsonOptic::from_path("outer.inner");
        let mut target: Value = json!({"a": {"b": "c"}});

        target.set(&optic, &Value::from(42));

        assert_eq!(target, json!({"outer": {"inner": 42}, "a": {"b": "c"}}))
    }

    #[test]
    fn setter_should_keep_array_contents() {
        let optic = JsonOptic::from_path("outer.inner.[2]");
        let mut target = json!({"outer": {"inner": [1, 2, 3]}});

        target.set(&optic, &Value::from(4));
        assert_eq!(target, json!({"outer": {"inner": [1, 2, 4]}}))
    }

    #[test]
    fn setter_should_set_fields_inside_arrays() {
        let optic = JsonOptic::from_path("outer.inner.[2].v");
        let mut target = json!({"outer": {"inner": [{"v": 1}, {"v": 2}, {"v": 3}]}});

        target.set(&optic, &Value::from(4));
        assert_eq!(target, json!({"outer": {"inner": [{"v": 1}, {"v": 2}, {"v": 4}]}}))
    }

    #[test]
    fn setter_should_write_at_correct_index() {
        let optic = JsonOptic::from_path("outer.inner.[1]");
        let mut target = json!({"outer": {"inner": [42]}});

        target.set(&optic, &Value::from(100));
        assert_eq!(target, json!({"outer": {"inner": [42, 100]}}))
    }

    #[test]
    fn setter_should_append_fields_inside_arrays() {
        let optic1 = JsonOptic::from_path("inner.[0].vv");
        let mut target1 = json!({"inner": [{"v": 1}, {"v": 2}, {"v": 3}]});
        target1.set(&optic1, &Value::from(4));
        assert_eq!(target1, json!({"inner": [{"v": 1, "vv": 4}, {"v": 2}, {"v": 3}]}));

        let optic2 = JsonOptic::from_path("inner.[1].vv");
        let mut target2 = json!({"inner": [{"v": 1}, {"v": 2}, {"v": 3}]});
        target2.set(&optic2, &Value::from(4));
        assert_eq!(target2, json!({"inner": [{"v": 1}, {"v": 2, "vv": 4}, {"v": 3}]}))
    }

    #[test]
    fn getter_should_extract_json() {
        let optic = JsonOptic::from_path("outer.inner");
        let target = json!({"outer": {"inner": {"a": {"b": "c"}}}});

        let result = target.get_all(&optic);
        assert_eq!(result, vec![&json!({"a": {"b": "c"}})])
    }

    #[test]
    fn getter_should_return_empty_json_if_there_is_no_subtree() {
        let optic = JsonOptic::from_path("outer.inner");
        let target = json!({"a": {"b": "c"}});

        let result = target.get_all(&optic);
        assert!(result.is_empty())
    }

    #[test]
    fn prune_should_do_nothing_if_there_is_no_subtree() {
        let optic = JsonOptic::from_path("outer.inner");
        let mut target = json!({"a": {"b": "c"}});

        target.prune(&optic);
        assert_eq!(target, json!({"a": {"b": "c"}}))
    }

    #[test]
    fn prune_should_cut_only_redundant_part_of_subtree() {
        let optic = JsonOptic::from_path("outer.inner");
        let mut target = json!({"outer": {"inner": 42, "other": {"b": "c"}}});

        target.prune(&optic);
        assert_eq!(target, json!({"outer": {"other": {"b": "c"}}}))
    }

    #[test]
    fn validate_should_return_true_if_subtree_exists() {
        let optic = JsonOptic::from_path("outer.inner");
        let target = json!({"outer": {"inner": 42}});

        assert!(target.validate(&optic))
    }

    #[test]
    fn validate_should_return_false_if_there_is_no_valid_subtree() {
        let optic = JsonOptic::from_path("outer.inner");
        let target = json!({"outer": {"other": {"b": "c"}}});

        assert!(!target.validate(&optic));
    }

    #[test]
    fn json_optic_correctly_deserializes() {
        let optic = serde_json::from_value::<JsonOptic>(json!("outer.inner"));

        assert!(optic.is_ok());
        assert_eq!("outer.inner", optic.ok().unwrap().to_string())
    }
}
