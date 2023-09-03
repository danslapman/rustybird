use crate::error::Error;
use crate::model::sql_json::Keyword as SqlKeyword;
use crate::utils::js::Jsn;
use crate::utils::js::optic::JsonOptic;
use diesel::{AppearsOnTable, Expression, SqlType, QueryResult};
use diesel::expression::{AsExpression, is_aggregate, ValidGrouping};
use diesel::pg::Pg;
use diesel::query_builder::{AstPass, QueryFragment, QueryId};
use diesel::result::Error as DieselError;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::{Bool, BigInt, Jsonb, Text};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Clone, Copy, SqlType)]
#[diesel(postgres_type(oid = 4072, array_oid = 4073))]
pub struct JsonPath(&'static str);

diesel::infix_operator!(Matches, " @@ ", backend: Pg);

impl ToSql<Text, Pg> for JsonPath
    where &'static str: ToSql<Text, Pg>
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        <str as ToSql<Text, Pg>>::to_sql(self.0, out)
    }
}

impl Debug for JsonPath {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

pub trait JsonbQueryMethods
    where
        Self: Expression<SqlType = Jsonb> + Sized
{
    fn exists<T: AsExpression<JsonPath>>(self, other: T) -> Matches<Self, T::Expression> {
        Matches::new(self, other.as_expression())
    }
}

impl<T: Expression<SqlType = Jsonb>> JsonbQueryMethods for T {}

#[derive(Debug)]
pub struct StateSpec(HashMap<JsonOptic, HashMap<SqlKeyword, Jsn>>);

impl StateSpec {
    pub fn from(spec: HashMap<JsonOptic, HashMap<SqlKeyword, Jsn>>) -> StateSpec {
        StateSpec(spec)
    }
}

impl QueryId for StateSpec {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}

impl Expression for StateSpec {
    type SqlType = JsonPath;
}

impl<GB> ValidGrouping<GB> for StateSpec {
    type IsAggregate = is_aggregate::Never;
}

impl<QS> AppearsOnTable<QS> for StateSpec where Self: Expression {}

fn query_builder_error(msg: &str) -> DieselError {
    DieselError::QueryBuilderError(Box::new(Error::new(msg.to_string())))
}

fn push_json_value<'a, 'b>(mut pass: AstPass<'a, 'b, Pg>, json_value: &'b Jsn) -> QueryResult<AstPass<'a, 'b, Pg>> {
    match json_value {
        Jsn::Bool(bool_val) => pass.push_bind_param::<Bool, _>(bool_val),
        Jsn::Signed(i_val) => pass.push_bind_param::<BigInt, _>(i_val),
        Jsn::String(str_val) => pass.push_bind_param::<Text, _>(str_val.as_str()),
        Jsn::Array(_) | Jsn::Object(_) => pass.push_bind_param::<Jsonb, _>(json_value),
        _ => Err(query_builder_error("Incorrect condition"))
    }?;

    Ok(pass)
}

impl QueryFragment<Pg> for StateSpec {
    fn walk_ast<'b>(&'b self, mut pass: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        for (optic, cond) in &self.0 {
            pass.push_sql(optic.to_json_path_string().as_str());

            for (kwd, val) in cond {
                match kwd {
                    SqlKeyword::Eq => {
                        pass.push_sql(" ?(@ == ");
                        pass = push_json_value(pass, val.into())?;
                        pass.push_sql(")");
                    }
                    SqlKeyword::NotEq => {
                        pass.push_sql(" ?(@ != ");
                        pass = push_json_value(pass, val.into())?;
                        pass.push_sql(")");
                    }
                    SqlKeyword::Less => {}
                    SqlKeyword::Lte => {}
                    SqlKeyword::Greater => {}
                    SqlKeyword::Gte => {}
                    SqlKeyword::Like => {}
                    SqlKeyword::StartsWith => {}
                    SqlKeyword::Exists => {}
                }
            }
        }

        Ok(())
    }
}