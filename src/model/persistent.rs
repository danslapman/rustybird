use crate::model::*;
use crate::predicate_dsl::json::JsonPredicate;
use crate::predicate_dsl::keyword::Keyword;
use crate::utils::js::optic::JsonOptic;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_json::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "mode")]
pub enum HttpStubRequest {
    JsonRequest {
        headers: HashMap<String, String>,
        query: HashMap<JsonOptic, HashMap<Keyword, Value>>,
        body: Value
    },
    RawRequest {
        headers: HashMap<String, String>,
        query: HashMap<JsonOptic, HashMap<Keyword, Value>>,
        body: String
    },
    JLensRequest {
        headers: HashMap<String, String>,
        query: HashMap<JsonOptic, HashMap<Keyword, Value>>,
        body: JsonPredicate
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "mode")]
pub enum HttpStubResponse {
    RawResponse {
        code: u16,
        headers: HashMap<String, String>,
        body: String,
        delay: Option<Duration>
    },
    JsonResponse {
        code: u16,
        headers: HashMap<String, String>,
        body: Value,
        delay: Option<Duration>,
        is_template: bool
    }
}


/*
final case class HttpStub(
    ...
    callback: Option[Callback],
    labels: Seq[String] = Seq.empty
)
 */

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::stub)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HttpStub {
    pub id: i32,
    pub created: DateTime<Utc>,
    //pub scope: Scope,
    pub times: Option<i64>,
    pub service_suffix: String,
    pub name: String,
    //pub method: HttpMethod,
    pub path: Option<String>,
    pub path_pattern: Option<String>,
    pub seed: Option<Value>,
    pub state: Option<Json<HashMap<JsonOptic, HashMap<Keyword, Value>>>>,
    pub request: Json<HttpStubRequest>,
    pub persist: Option<Json<HashMap<JsonOptic, Value>>>,
    pub response: Json<HttpStubResponse>
}