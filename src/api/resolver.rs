use crate::model::*;
use crate::error::Error;
use crate::model::persistent::HttpStub;
use crate::utils::pg::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use log::{info, warn};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone)]
pub struct StubResolver {
    pool: Pool<ConnectionManager<PgConnection>>
}

impl StubResolver {
    fn find_stub_and_state(&self, in_scope: Scope, with_method: HttpMethod, with_path: String, with_headers: HashMap<String, String>, query_object: Value) -> Result<Option<HttpStub>, Error> {
        use crate::schema::stub::dsl::*;

        let mut conn = self.pool.get()?;

        info!("Searching searching stubs for {:?} of scope {:?}", path, scope);

        //TODO: add "times" filter for "countdown" scope
        let candidates0: Vec<HttpStub> = stub.filter(
            scope.eq(in_scope)
                .and(method.eq(with_method))
                .and(path.eq(with_path.clone()).or(ilike_rev(with_path, path_pattern)))
        ).load(&mut conn)?;

        if candidates0.is_empty() {
            info!("Stubs for {:?} not found in scope {:?}", path, scope);
            return Err(Error::new(format!("Stubs for {:?} not found in scope {:?}", path, scope)));
        }

        info!("Candidates for {:?} in {:?} are: {:?}", path, scope, candidates0.iter().map(|c| c.id).collect::<Vec<_>>());

        let candidates1 = candidates0.into_iter().filter(|s| s.request.0.check_query_params(query_object.clone())).collect::<Vec<_>>();

        if candidates1.is_empty() {
            info!("There are no {:?} candidates in scope {:?} after query parameters check", path, scope);
            return Err(Error::new(format!("There are no {:?} candidates in scope {:?} after query parameters check", path, scope)));
        }

        info!("Candidates for {:?} in {:?} after query parameters check are: {:?}", path, scope, candidates1.iter().map(|c| c.id).collect::<Vec<_>>());

        Ok(None)
    }
}