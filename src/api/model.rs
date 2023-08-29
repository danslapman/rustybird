use crate::model::*;
use crate::utils::js::optic::JsonOptic;
use regex::Regex;
use serde::{Deserialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CreateStubRequest {
    pub scope: Scope,
    pub times: Option<u32>,
    pub name: String,
    pub method: HttpMethod,
    pub path: Option<String>,
    #[serde(with = "serde_regex")]
    pub path_pattern: Option<Regex>,
    pub state: Option<HashMap<JsonOptic, HashMap<String, HashMap<String, Value>>>>,
    pub request: persistent::HttpStubRequest,
    pub persist: Option<HashMap<JsonOptic, Value>>,
    pub response: persistent::HttpStubResponse
}