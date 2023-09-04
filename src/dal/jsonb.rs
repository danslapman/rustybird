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
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Clone, Copy, SqlType)]
#[diesel(postgres_type(oid = 4072, array_oid = 4073))]
pub struct JsonPath(&'static str);

diesel::infix_operator!(Exists, " @? ", backend: Pg);
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
    fn exists<T: AsExpression<JsonPath>>(self, other: T) -> Exists<Self, T::Expression> {
        Exists::new(self, other.as_expression())
    }

    fn matches<T: AsExpression<JsonPath>>(self, other: T) -> Matches<Self, T::Expression> {
        Matches::new(self, other.as_expression())
    }
}

impl<T: Expression<SqlType = Jsonb>> JsonbQueryMethods for T {}

#[derive(Debug)]
pub struct Predicate(JsonOptic, HashMap<SqlKeyword, Jsn>);

impl Predicate {
    pub fn from(optic: JsonOptic, spec: HashMap<SqlKeyword, Value>) -> Predicate {
        Predicate(optic, spec.into_iter()
            .map(|(kx, vx)| (kx, Into::<Jsn>::into(vx)))
            .collect::<HashMap<_, _>>())
    }
}

impl QueryId for Predicate {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}

impl Expression for Predicate {
    type SqlType = JsonPath;
}

impl<GB> ValidGrouping<GB> for Predicate {
    type IsAggregate = is_aggregate::Never;
}

impl<QS> AppearsOnTable<QS> for Predicate where Self: Expression {}

impl QueryFragment<Pg> for Predicate {
    fn walk_ast<'b>(&'b self, mut pass: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        pass.push_sql("'");
        pass.push_sql(self.0.to_json_path_string().as_str());

        for (kwd, val) in self.1.iter() {
            match kwd {
                SqlKeyword::Eq => {
                    pass.push_sql(" ?(@ == ");
                    pass = push_json_value(pass, val)?;
                    pass.push_sql(")");
                }
                SqlKeyword::NotEq => {
                    pass.push_sql(" ?(@ != ");
                    pass = push_json_value(pass, val)?;
                    pass.push_sql(")");
                }
                SqlKeyword::Less => {
                    pass.push_sql(" ?(@ < ");
                    match val {
                        Jsn::Signed(_) | Jsn::Unsigned(_) | Jsn::Float(_) => pass = push_json_value(pass, val)?,
                        _ => return Err(query_builder_error("Incorrect argument for '<'"))
                    }
                    pass.push_sql(")");
                }
                SqlKeyword::Lte => {
                    pass.push_sql(" ?(@ <= ");
                    match val {
                        Jsn::Signed(_) | Jsn::Unsigned(_) | Jsn::Float(_) => pass = push_json_value(pass, val)?,
                        _ => return Err(query_builder_error("Incorrect argument for '<='"))
                    }
                    pass.push_sql(")");
                }
                SqlKeyword::Greater => {
                    pass.push_sql(" ?(@ > ");
                    match val {
                        Jsn::Signed(_) | Jsn::Unsigned(_) | Jsn::Float(_) => pass = push_json_value(pass, val)?,
                        _ => return Err(query_builder_error("Incorrect argument for '>'"))
                    }
                    pass.push_sql(")");
                }
                SqlKeyword::Gte => {
                    pass.push_sql(" ?(@ >= ");
                    match val {
                        Jsn::Signed(_) | Jsn::Unsigned(_) | Jsn::Float(_) => pass = push_json_value(pass, val)?,
                        _ => return Err(query_builder_error("Incorrect argument for '>='"))
                    }
                    pass.push_sql(")");
                }
                SqlKeyword::Like => {
                    pass.push_sql(" ?(@ like_regex ");
                    match val {
                        Jsn::String(_) => pass = push_json_value(pass, val)?,
                        _ => return Err(query_builder_error("Incorrect argument for 'like_regex'"))
                    }
                    pass.push_sql(")");
                }
                SqlKeyword::StartsWith => {
                    pass.push_sql(" ?(@ starts with ");
                    match val {
                        Jsn::String(_) => pass = push_json_value(pass, val)?,
                        _ => return Err(query_builder_error("Incorrect argument for 'starts with'"))
                    }
                    pass.push_sql(")");
                }
            }
        }
        pass.push_sql("'");

        Ok(())
    }
}

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

#[cfg(test)]
mod jsonb_tests {
    use crate::dal::jsonb::{JsonPath, JsonbQueryMethods, Predicate};
    use crate::model::sql_json::Keyword;
    use crate::schema::state::dsl::*;
    use crate::utils::js::optic::JsonOptic;
    use diesel::prelude::*;
    use diesel::pg::Pg;
    use diesel::query_builder::debug_query;
    use serde_json::{json, Value};
    use std::collections::HashMap;

    #[test]
    fn check_equals_spec_sql() {
        let optic = JsonOptic::from_path("a.b");
        let spec = serde_json::from_value::<HashMap<Keyword, Value>>(json!({"==": 42})).ok().unwrap();
        let sql = debug_query::<Pg, _>(&state.filter(&data.exists((&Predicate::from(optic, spec)).into_sql::<JsonPath>()))).to_string();
        assert_eq!(sql, r#"SELECT "state"."id", "state"."created", "state"."data" FROM "state" WHERE "state"."data" @? '$.a.b ?(@ == $1)' -- binds: [42]"#)
    }
}