use std::fmt::Display;

pub struct Error {
    pub cause: String
}

impl Error {
    pub fn new(message: String) -> Error {
        Error { cause: message }
    }

    pub fn from<T: Display>(underlying: T) -> Error {
        Error { cause: format!("{}", underlying) }
    }
}