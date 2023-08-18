use crate::predicate_dsl::keyword::Keyword;
use crate::utils::{IntoBD, IntoUSize};
use crate::utils::js::optic::JsonOptic;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

type Spec = HashMap<JsonOptic, HashMap<Keyword, Value>>;
type Condition<'r> = (&'r Keyword, &'r Value);

pub struct JsonPredicate {
    definition: Spec
}

impl JsonPredicate {
    pub fn validate(&self, json: Value) -> bool {
        false
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