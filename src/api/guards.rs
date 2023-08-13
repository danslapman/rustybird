use std::collections::HashMap;
use rocket::http::HeaderMap;
use rocket::request::{Outcome, Request, FromRequest};

pub struct RequestHeaders<'r>(&'r HeaderMap<'r>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestHeaders<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(RequestHeaders(request.headers()))
    }
}

pub struct QueryParameters<'r>(HashMap::<&'r str, &'r str>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for QueryParameters<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let mut res = HashMap::<&'r str, &'r str>::new();

        for vf in request.query_fields() {
            res.insert(vf.name.as_name(), vf.value);
        }

        Outcome::Success(QueryParameters(res))
    }
}