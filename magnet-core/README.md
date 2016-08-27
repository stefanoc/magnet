```rust
extern crate hyper;
extern crate magnet_core;

use hyper::server::Server;
use magnet_core::*;

struct Hello;

impl Responder for Hello {
    fn call(&self, _stack: &Stack, _request: &Request) -> MagnetResult<Option<Response>> {
        let resp = Response::build(Status::Ok).html("hello").finalize();
        Ok(Some(resp))
    }
}

fn main() {
    let mut root = Stack::new();
    root.add(Hello);
    Server::http("0.0.0.0:3000").unwrap().handle(root).unwrap();
}
```
