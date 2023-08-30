use crate::error::Error;
use crate::model::persistent::*;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

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