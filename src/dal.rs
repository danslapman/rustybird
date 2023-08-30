use crate::error::Error;
use crate::model::persistent::*;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use serde_json::Value;

pub mod error;

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
}