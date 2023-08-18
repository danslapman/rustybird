use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive, Zero};
use serde_json::Number;
use crate::utils::{IntoBD, IntoUSize};

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