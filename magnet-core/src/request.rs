use hyper;
use typemap;
use super::Error;
pub use hyper::method::Method;

#[derive(Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    data: typemap::ShareCloneMap,
}

impl Request {
    fn new(method: Method, path: String) -> Request {
        Request {
            method: method,
            path: path,
            data: typemap::ShareCloneMap::custom(),
        }
    }

    pub fn build(r: hyper::server::Request) -> Result<Request, Error> {
        use hyper::uri::RequestUri::*;

        if let AbsolutePath(path) = r.uri {
            Ok(Request::new(r.method, path))
        } else {
            Err(Error::UnsupportedRequestPathFormat)
        }
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn get<T>(&self) -> Option<&T::Value>
        where T: typemap::Key,
              T::Value: Clone + Send + Sync
    {
        self.data.get::<T>()
    }

    pub fn set<T>(&mut self, value: T::Value)
        where T: typemap::Key,
              T::Value: Clone + Send + Sync
    {
        self.data.insert::<T>(value);
    }
}
