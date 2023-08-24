pub mod persistent;

pub enum Scope {
    Persistent,
    Ephemeral,
    Countdown
}

pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Connect,
    Options,
    Trace,
    Patch
}