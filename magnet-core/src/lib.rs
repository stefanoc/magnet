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

pub enum MagnetError {
    Generic(String),
    UnsupportedRequestPathFormat,
}

pub type MagnetResult<T> = Result<T, MagnetError>;
