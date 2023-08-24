use crate::model::*;
use crate::model::persistent::HttpStub;

pub struct StubResolver {}

impl StubResolver {
    fn find_stub_and_state(method: HttpMethod) -> Option<HttpStub> {
        None
    }
}