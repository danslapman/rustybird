use crate::api::model::*;
use crate::dal::*;
use crate::error::Error;
use crate::model::persistent;
use chrono::Utc;
use diesel_json::Json;

#[derive(Clone)]
pub struct AdminApiHandler {
    stub_dao: StubDao,
    state_dao: StateDao
}

impl AdminApiHandler {
    pub fn new(stub_dao: StubDao, state_dao: StateDao) -> AdminApiHandler {
        AdminApiHandler { stub_dao, state_dao }
    }

    pub async fn create_stub(&self, req_stub: CreateStubRequest) -> Result<bool, Error> {
        let new_stub = persistent::NewHttpStub {
            created: Utc::now(),
            scope: req_stub.scope,
            times: req_stub.times.map(|u| u.into()),
            service_suffix: "".to_string(),
            name: req_stub.name,
            method: req_stub.method,
            path: req_stub.path,
            path_pattern: req_stub.path_pattern.map(|rx| rx.to_string()),
            seed: None,
            state: req_stub.state.map(|spec| Json::new(spec)),
            request: Json::new(req_stub.request),
            persist: None,
            response: Json::new(req_stub.response),
            callback: req_stub.callback.map(|c| Json::new(c))
        };

        self.stub_dao.insert_stub(new_stub).await.map(|res| res > 0)
    }

    pub async fn fetch_states(&self, request: SearchRequest) -> Result<Vec<persistent::State>, Error> {
        self.state_dao.find_by_spec(request.query).await
    }
}