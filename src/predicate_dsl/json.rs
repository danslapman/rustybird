use crate::predicate_dsl::keyword::Keyword;
use crate::utils::{IntoBD, IntoUSize};
use crate::utils::js::optic::{JsonOptic, ValueExt};
use regex::Regex;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde_json::Value;
use std::collections::HashMap;

type Spec = HashMap<JsonOptic, HashMap<Keyword, Value>>;
type Condition<'r> = (&'r Keyword, &'r Value);

pub struct JsonPredicate {
    definition: Spec
}

impl JsonPredicate {
    pub fn validate(&self, json: Value) -> Result<bool, PredicateConstructionError<'_>> {
        let mut result: Vec<Result<bool, Condition<'_>>> = vec![];

        for (jo, conds) in self.definition.iter() {
            let all_data = json.get_all(jo);
            let data = all_data.first().unwrap_or(&&Value::Null);

            for (kwd, etalon) in conds.iter() {
                result.push(JsonPredicate::validate_one(kwd, etalon, *data));
            }
        }

        let (oks, errs): (Vec<_>, Vec<_>) = result.into_iter().partition(|el| el.is_ok());

        if errs.len() == 0 {
            Ok(oks.into_iter().filter_map(|el| el.ok()).all(|el| el))
        } else {
            Err(PredicateConstructionError { problems: errs.into_iter().filter_map(|el| el.err()).collect() })
        }
    }

    fn validate_one<'r>(kwd: &'r Keyword, etalon: &'r Value, value: &Value) -> Result<bool, Condition<'r>> {
        match (kwd, etalon, value) {
            (Keyword::Equals, v_eq, val) => Ok(v_eq == val),
            (Keyword::NotEq, v_neq, val) => Ok(v_neq != val),
            (Keyword::Greater, Value::Number(l_b), Value::Number(nv)) => Ok(nv.to_big_decimal() > l_b.to_big_decimal()),
            (Keyword::Gte, Value::Number(l_b), Value::Number(nv)) => Ok(nv.to_big_decimal() >= l_b.to_big_decimal()),
            (Keyword::Less, Value::Number(u_b), Value::Number(nv)) => Ok(nv.to_big_decimal() < u_b.to_big_decimal()),
            (Keyword::Lte, Value::Number(u_b), Value::Number(nv)) => Ok(nv.to_big_decimal() <= u_b.to_big_decimal()),
            (Keyword::Rx, Value::String(rx), Value::String(s)) => match Regex::new(rx) {
                Ok(regex) => Ok(regex.is_match(s)),
                _ => Err((kwd, etalon))
            },
            (Keyword::Size, Value::Number(size), Value::String(s)) => Ok(s.len() == size.to_usize()),
            (Keyword::Size, Value::Number(size), Value::Array(v)) => Ok(v.len() == size.to_usize()),
            (Keyword::Exists, Value::Bool(true), val) => Ok(!val.is_null()),
            (Keyword::Exists, Value::Bool(false), val) => Ok(val.is_null()),
            (Keyword::In, Value::Array(possible), Value::Array(vals)) => Ok(possible.iter().any(|pv| vals.contains(pv))),
            (Keyword::In, Value::Array(possible), val) => Ok(possible.contains(val)),
            (Keyword::NotIn, Value::Array(impossible), Value::Array(vals)) => Ok(impossible.iter().all(|iv| !vals.contains(iv))),
            (Keyword::NotIn, Value::Array(impossible), val) => Ok(!impossible.contains(val)),
            (Keyword::AllIn, Value::Array(mandatory), Value::Array(vals)) => Ok(mandatory.iter().all(|mv| vals.contains(mv))),
            (k, v, _) => Err((k, v))
        }
    }
}

impl Serialize for JsonPredicate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.definition.serialize(serializer)
    }
}

impl <'de> Deserialize<'de> for JsonPredicate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        Spec::deserialize(deserializer).map(|spec| JsonPredicate { definition: spec })
    }
}

fn validate_condition<'r>(kwd: &'r Keyword, etalon: &'r Value) -> bool {
    match (kwd, etalon) {
        (Keyword::Equals | Keyword::NotEq, _) => true,
        (Keyword::Greater | Keyword::Gte | Keyword::Less | Keyword::Lte, Value::Number(_)) => true,
        (Keyword::Rx, Value::String(rx)) if Regex::new(rx).is_ok() => true,
        (Keyword::Size, Value::Number(_)) => true,
        (Keyword::Exists, Value::Bool(_)) => true,
        (Keyword::In | Keyword::NotIn | Keyword::AllIn, Value::Array(_)) => true,
        (_, _) => false
    }
}

pub struct PredicateConstructionError<'r> {
    pub problems: Vec<Condition<'r>>
}