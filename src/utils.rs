use bigdecimal::BigDecimal;

pub mod js;
pub mod pg;

pub trait IntoBD {
    fn to_big_decimal(self) -> BigDecimal;
}

pub trait IntoUSize {
    fn to_usize(self) -> usize;
}