use crate::dal::jsonb::{JsonPath, JsonbQueryMethods, Predicate};
use crate::error::Error;
use crate::model::persistent::*;
use crate::model::sql_json::{Keyword as SqlKeyword};
use crate::utils::js::optic::JsonOptic;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use serde_json::Value;
use std::collections::HashMap;

pub mod error;
pub mod jsonb;

#[derive(Clone)]
pub struct StubDao {
    pool: Pool<ConnectionManager<PgConnection>>
}

impl StubDao {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> StubDao {
        StubDao { pool }
    }

    pub async fn insert_stub(&self, new_stub: NewHttpStub) -> Result<usize, Error> {
        use crate::schema::stub::dsl::*;

        let mut conn = self.pool.get()?;

        let res = diesel::insert_into(stub)
            .values(&new_stub)
            .execute(&mut conn)?;

        Ok(res)
    }
}

#[derive(Clone)]
pub struct StateDao {
    pool: Pool<ConnectionManager<PgConnection>>
}

impl StateDao {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> StateDao {
        StateDao { pool }
    }

    pub async fn create_state(&self, state_data: Value) -> Result<usize, Error> {
        use crate::schema::state::dsl::*;

        let mut conn = self.pool.get()?;

        let new_state = NewState { created: Utc::now(), data: state_data };

        let res = diesel::insert_into(state)
            .values(&new_state)
            .execute(&mut conn)?;

        Ok(res)
    }

    pub async fn find_by_spec(&self, spec: HashMap<JsonOptic, HashMap<SqlKeyword, Value>>) -> Result<Vec<State>, Error> {
        use crate::schema::state::dsl::*;

        let mut conn = self.pool.get()?;

        let mut query = state.into_boxed();

        let predicates = spec.into_iter().map(|(optic, spec)| Predicate::from(optic, spec)).collect::<Vec<_>>();

        for pred in predicates {
            query = query.filter(data.exists(pred.into_sql::<JsonPath>()));
        }

        //println!("{:#?}", diesel::query_builder::debug_query::<diesel::pg::Pg, _>(&query).to_string());

        let res = query.load(&mut conn)?;

        Ok(res)
    }
}