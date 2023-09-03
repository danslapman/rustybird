use crate::utils::js::Jsn;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Jsonb;
use std::io::Write;

impl ToSql<Jsonb, Pg> for Jsn {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        write!(out, "{}", self).map(|_| IsNull::No).map_err(Into::into)
    }
}