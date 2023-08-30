use crate::model::*;
use crate::predicate_dsl::keyword::Keyword;
use crate::utils::js::optic::JsonOptic;
use regex::Regex;
use serde::{Deserialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CreateStubRequest {
    pub scope: Scope,
    #[serde(default)]
    pub times: Option<u32>,
    pub name: String,
    pub method: HttpMethod,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(with = "serde_regex")]
    #[serde(default)]
    pub path_pattern: Option<Regex>,
    #[serde(default)]
    pub state: Option<HashMap<JsonOptic, HashMap<Keyword, Value>>>,
    pub request: persistent::HttpStubRequest,
    #[serde(default)]
    pub persist: Option<HashMap<JsonOptic, Value>>,
    pub response: persistent::HttpStubResponse,
    pub callback: Option<persistent::Callback>
}