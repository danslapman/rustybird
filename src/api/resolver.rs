use crate::model::persistent::HttpStub;
use rocket::http::Method;

pub struct StubResolver {}

impl StubResolver {
    fn find_stub_and_state(method: Method) -> Option<HttpStub> {
        None
    }
}