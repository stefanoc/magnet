pub use hyper::status::StatusCode as Status;
use super::header::{self, Headers, Header, HeaderFormat};

pub struct Response {
    pub status: Status,
    pub headers: Headers,
    pub body: String,
}

pub struct ResponseBuilder {
    status: Status,
    headers: Headers,
    body: Option<String>,
}

impl Response {
    pub fn build(status: Status) -> ResponseBuilder {
        ResponseBuilder {
            status: status,
            body: None,
            headers: Headers::new(),
        }
    }
}

impl ResponseBuilder {
    pub fn html<T>(mut self, body: T) -> ResponseBuilder
        where T: Into<String>
    {
        self.body = Some(body.into());
        self.headers.set(header::ContentType::html());
        self
    }

    pub fn body<T>(mut self, body: T) -> ResponseBuilder
        where T: Into<String>
    {
        self.body = Some(body.into());
        self
    }

    pub fn header<T>(mut self, header: T) -> ResponseBuilder
        where T: Header + HeaderFormat
    {
        self.headers.set(header);
        self
    }

    pub fn end(self) -> Response {
        Response {
            status: self.status,
            body: self.body.unwrap_or("".into()),
            headers: self.headers,
        }
    }
}
