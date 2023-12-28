use crate::utils::js::Jsn;
use diesel::*;
use diesel::expression::*;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::{Jsonb, SqlType};
use std::io::Write;

impl ToSql<Jsonb, Pg> for Jsn {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        write!(out, "{}", self).map(|_| IsNull::No).map_err(Into::into)
    }
}

infix_operator!(LikeInsense, " ~* ", backend: Pg);

pub fn ilike_rev<T, U, ST>(left: T, right: U) -> LikeInsense<T::Expression, U> where
     T: AsExpression<ST>,
     U: Expression<SqlType = ST>,
     ST: SqlType + TypedExpressionType,
{
    LikeInsense::new(left.as_expression(), right) 
}

/*
#[derive(Debug, Copy, Clone, QueryId, Default, DieselNumericOps, ValidGrouping)]
pub struct Enclosured<T>(pub T);

impl<T: Expression> Expression for Enclosured<T> {
    type SqlType = T::SqlType;
}

impl<T, DB> QueryFragment<DB> for Enclosured<T>
where
    T: QueryFragment<DB>,
    DB: Backend,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, DB>) -> QueryResult<()> {
        out.push_sql("(");
        self.0.walk_ast(out.reborrow())?;
        out.push_sql(")");
        Ok(())
    }
}

impl_selectable_expression!(Enclosured<T>);
*/