use crate::model::*;
use crate::model::sql_json::Keyword as SqlKeyword;
use crate::predicate_dsl::json::JsonPredicate;
use crate::predicate_dsl::keyword::Keyword;
use crate::utils::js::optic::JsonOptic;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_autoincrement_new_struct::prelude::*;
use diesel_json::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "mode")]
pub enum HttpStubRequest {
    #[serde(rename = "no_body")]
    RequestWithoutBody {
        headers: HashMap<String, String>,
        #[serde(default = "HashMap::new")]
        query: HashMap<JsonOptic, HashMap<Keyword, Value>>
    },
    #[serde(rename = "json")]
    JsonRequest {
        headers: HashMap<String, String>,
        #[serde(default = "HashMap::new")]
        query: HashMap<JsonOptic, HashMap<Keyword, Value>>,
        body: Value
    },
    #[serde(rename = "raw")]
    RawRequest {
        headers: HashMap<String, String>,
        #[serde(default = "HashMap::new")]
        query: HashMap<JsonOptic, HashMap<Keyword, Value>>,
        body: String
    },
    #[serde(rename = "jlens")]
    JLensRequest {
        headers: HashMap<String, String>,
        #[serde(default = "HashMap::new")]
        query: HashMap<JsonOptic, HashMap<Keyword, Value>>,
        body: JsonPredicate
    }
}

impl HttpStubRequest {
    pub fn check_headers(&self, hs: HashMap<String, String>) -> bool {
        self.headers().iter().all(|(k, v)| hs.get(k).is_some_and(|vx| vx.to_lowercase() == v.to_lowercase()))
    }

    pub fn check_query_params(&self, params: Value) -> bool {
        if self.query().is_empty() {
            true
        } else {
            JsonPredicate::from_spec(self.query().clone()).validate(params).unwrap_or(false)
        }
    }

    fn headers(&self) -> &HashMap<String, String> {
        match self {
            HttpStubRequest::RequestWithoutBody { headers, .. } => headers,
            HttpStubRequest::JsonRequest { headers, .. } => headers,
            HttpStubRequest::RawRequest { headers, .. } => headers,
            HttpStubRequest::JLensRequest { headers, .. } => headers,
        }
    }

    fn query(&self) -> &HashMap<JsonOptic, HashMap<Keyword, Value>> {
        match self {
            HttpStubRequest::RequestWithoutBody { query, .. } => query,
            HttpStubRequest::JsonRequest { query, .. } => query,
            HttpStubRequest::RawRequest { query, .. } => query,
            HttpStubRequest::JLensRequest { query, .. } => query,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "mode")]
pub enum HttpStubResponse {
    #[serde(rename = "raw")]
    RawResponse {
        code: u16,
        headers: HashMap<String, String>,
        body: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        delay: Option<Duration>
    },
    #[serde(rename = "json")]
    JsonResponse {
        code: u16,
        headers: HashMap<String, String>,
        body: Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        delay: Option<Duration>,
        is_template: bool
    }
}

#[apply(NewInsertable!)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::stub)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HttpStub {
    pub id: i32,
    pub created: DateTime<Utc>,
    pub scope: Scope,
    pub times: Option<i64>,
    pub service_suffix: String,
    pub name: String,
    pub method: HttpMethod,
    pub path: Option<String>,
    pub path_pattern: Option<String>,
    pub seed: Option<Value>,
    pub state: Option<Json<HashMap<JsonOptic, HashMap<SqlKeyword, Value>>>>,
    pub request: Json<HttpStubRequest>,
    pub persist: Option<Json<HashMap<JsonOptic, Value>>>,
    pub response: Json<HttpStubResponse>,
    pub callback: Option<Json<Callback>>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CallbackResponseMode {
    Json
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "mode")]
pub enum CallbackRequest {
    #[serde(rename = "no_body")]
    CallbackRequestWithoutBody {
        url: String,
        method: HttpMethod,
        headers: HashMap<String, String>
    },
    #[serde(rename = "raw")]
    RawCallbackRequest {
        url: String,
        method: HttpMethod,
        headers: HashMap<String, String>,
        body: String
    },
    #[serde(rename = "json")]
    JsonCallbackRequest {
        url: String,
        method: HttpMethod,
        headers: HashMap<String, String>,
        body: Value
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Callback {
    HttpCallback {
        request: CallbackRequest,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        response_mode: Option<CallbackResponseMode>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        persist: Option<HashMap<JsonOptic, Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        callback: Option<Box<Callback>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        delay: Option<Duration>
    }
}

#[apply(NewInsertable!)]
#[derive(Queryable, Selectable, Serialize, QueryableByName)]
#[diesel(table_name = crate::schema::state)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct State {
    pub id: i32,
    pub created: DateTime<Utc>,
    pub data: Value
}