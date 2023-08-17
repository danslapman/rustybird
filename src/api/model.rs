use crate::model::*;
use crate::utils::js::optic::JsonOptic;
use regex::Regex;
use rocket::http::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
#[serde(tag = "mode")]
pub enum HttpStubRequest {
    Json {
        headers: HashMap<String, String>,
        query: HashMap<JsonOptic, HashMap<String, Value>>,
        body: Value
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "mode")]
pub enum HttpStubResponse {
    Raw {
        code: i64,
        headers: HashMap<String, String>,
        body: String,
        delay: Option<Duration>
    }
}

pub struct CreateStubRequest {
    scope: Scope,
    times: Option<i64>,
    name: String,
    method: Method,
    path: Option<String>,
    path_pattern: Option<Regex>,
    state: Option<HashMap<JsonOptic, HashMap<String, HashMap<String, Value>>>>,
    request: HttpStubRequest,
    persist: Option<HashMap<JsonOptic, Value>>,
    response: HttpStubResponse
}