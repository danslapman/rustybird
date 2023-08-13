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