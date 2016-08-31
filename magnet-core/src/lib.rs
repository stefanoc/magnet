extern crate hyper;
extern crate typemap;

mod request;
mod response;
mod stack;

pub mod header {
    pub use hyper::header::*;
}

pub mod ext {
    pub use typemap::Key;
}

pub use request::{Request, Method};
pub use response::{Response, Status};
pub use stack::{Stack, Before, After, Responder};

#[derive(Debug)]
pub enum Error {
    Generic(String),
    UnsupportedRequestPathFormat,
}

impl Error {
    pub fn message(&self) -> &str {
        match *self {
            Error::Generic(ref msg) => msg,
            Error::UnsupportedRequestPathFormat => "Only AbsoluteUri request paths are supported",
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.message()
    }
}

pub type MagnetResult<T> = Result<T, Error>;
