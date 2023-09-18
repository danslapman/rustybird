use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::Value;
use crate::utils::js::optic::{JsonOptic, ValueExt};

static JSON_OPTIC_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$([:~])?\{([\p{L}\d\.\[\]\-_]+)\}").unwrap());

pub struct JsonTemplater {
    values: Value
}

impl JsonTemplater {
    pub fn new(values: Value) -> JsonTemplater {
        JsonTemplater { values }
    }

    pub fn make_patcher_fn(&self, defn: &str) -> Option<impl Fn(&mut Value)> {
        if let Some(caps) = JSON_OPTIC_PATTERN.captures(defn) {
            let modifier = caps.get(1).map(|m| m.as_str());
            let path = &caps[2];
            let optic = JsonOptic::from_path(path);

            if self.values.validate(&optic) {
                let new_value = self.values.get_all(&optic)[0].clone();

                return Some(move |target: &mut Value| *target = new_value.clone())
            }
        }

        None
    }
}

pub trait JsonTransformations {
    fn update_in_place_by_fn(&mut self, modify: fn(&mut Value));
    fn update_in_place_by_closure(&mut self, modify: &dyn Fn(&mut Value));
    fn substitute_in_place(&mut self, values: Value);
}

impl JsonTransformations for Value {
    fn update_in_place_by_fn(&mut self, modify: fn(&mut Value)) {
        match self {
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => modify(self),
            Value::Array(vs) => vs.iter_mut().for_each(|el| el.update_in_place_by_fn(modify)),
            Value::Object(kvs) => kvs.iter_mut().for_each(|(_, val)| val.update_in_place_by_fn(modify))
        }
    }

    fn update_in_place_by_closure(&mut self, modify: &dyn Fn(&mut Value)) {
        match self {
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => modify(self),
            Value::Array(vs) => vs.iter_mut().for_each(|el| el.update_in_place_by_closure(modify)),
            Value::Object(kvs) => kvs.iter_mut().for_each(|(_, val)| val.update_in_place_by_closure(modify))
        }
    }

    fn substitute_in_place(&mut self, values: Value) {
        let templater = JsonTemplater::new(values);

        let upd = |vx: &mut Value| {
            match vx {
                Value::String(s) => {
                    if let Some(patcher) = templater.make_patcher_fn(&s) {
                        patcher(vx)
                    }
                },
                _ => ()
            }
        };

        self.update_in_place_by_closure(&upd);
    }
}

#[cfg(test)]
mod json_templater_tests {
    use serde_json::{json, Value};
    use crate::utils::transformations::js::*;

    #[test]
    fn fill_template() {
        let mut template: Value = json!({
            "description": "${description}",
            "topic" : "${extras.topic}",
            "comment" : "${extras.comments.[0].text}",
            "meta" : {
                "field1" : "${extras.fields.[0]}"
            },
        });

        let data: Value = json!({
            "description": "Some description",
            "extras": {
                "fields": ["f1", "f2"],
                "topic": "Main topic",
                "comments": [{"text": "First nah!"}, {"text": "Okay"}]
            }
        });

        template.substitute_in_place(data);

        assert_eq!(template,
                   json!({
                       "description": "Some description",
                       "topic" : "Main topic",
                       "comment" : "First nah!",
                       "meta" : {
                           "field1": "f1"
                       }
        }))
    }
}