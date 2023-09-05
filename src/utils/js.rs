use crate::utils::{IntoBD, IntoUSize};
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive, Zero};
use serde_json::{Number, Value};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

pub mod optic;

impl IntoBD for &Number {
    fn to_big_decimal(self) -> BigDecimal {
        self.as_i64().map(|i| BigDecimal::from(i))
            .or(self.as_u64().map(|u| BigDecimal::from(u)))
            .or(self.as_f64().and_then(|f| BigDecimal::from_f64(f)))
            .unwrap_or(BigDecimal::zero())
    }
}

impl IntoUSize for &Number {
    fn to_usize(self) -> usize {
        self.as_i64().and_then(|i| i.to_usize())
            .or(self.as_u64().and_then(|u| u.to_usize()))
            .or(self.as_f64().and_then(|f| f.to_usize()))
            .unwrap_or(0)
    }
}

/// This Json representation explicitly owns number representations
pub enum Jsn {
    Null,
    Bool(bool),
    Signed(i64),
    Float(f64),
    String(String),
    Array(Vec<Jsn>),
    Object(HashMap<String, Jsn>),
}

impl Jsn {
    pub fn is_string(&self) -> bool {
        match self {
            Jsn::String(_) => true,
            _ => false
        }
    }
}

impl From<Value> for Jsn {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Jsn::Null,
            Value::Bool(boolean) => Jsn::Bool(boolean),
            Value::Number(n) if n.is_i64() => Jsn::Signed(n.as_i64().unwrap()),
            Value::Number(n) if n.is_u64() => {
                if n.as_u64().unwrap() <= i64::MAX as u64 {
                    Jsn::Signed(n.as_u64().unwrap() as i64)
                } else {
                    Jsn::Float(n.as_f64().unwrap())
                }
            },
            Value::Number(n) if n.is_f64() => Jsn::Float(n.as_f64().unwrap()),
            Value::String(str) => Jsn::String(str),
            Value::Array(els) => Jsn::Array(els.iter().map(|el| el.clone().clone().into()).collect::<Vec<_>>()),
            Value::Object(flds) => Jsn::Object(flds.iter().map(|(k, v)| (k.clone(), v.clone().into())).collect::<HashMap<_, _>>()),
            _ => panic!("This should never happen")
        }
    }
}

impl Display for Jsn {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Jsn::Null => write!(fmt, "{}", "null"),
            Jsn::Bool(b) => write!(fmt, "{}", b),
            Jsn::Signed(i) => write!(fmt, "{}", i),
            Jsn::Float(f) => write!(fmt, "{}", f),
            Jsn::String(s) => write!(fmt, "\"{}\"", s.escape_default()),
            Jsn::Array(els) => write!(fmt, "{:?}", els),
            Jsn::Object(ps) => {
                write!(fmt, "{{")?;
                for (k,v) in ps {
                    write!(fmt, "\"{}\": {:?}", k.escape_default(), v)?;
                }
                write!(fmt, "}}")
            }
        }
    }
}

impl Debug for Jsn {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self)
    }
}

#[cfg(test)]
mod js_tests {
    use crate::utils::js::Jsn;
    use serde_json::json;

    #[test]
    fn check_jsn_debug() {
        let jsn: Jsn = json!({"a": ["b", 3, false, null]}).into();
        let sut = format!("{:?}", jsn);

        assert_eq!(sut, r#"{"a": ["b", 3, false, null]}"#)
    }
}