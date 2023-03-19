use std::fmt::{Debug, Display, Formatter};

pub struct Error {
    pub reason: String,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred: {:?}", self.reason)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred: {:?}", self.reason)
    }
}

impl std::error::Error for Error {}
