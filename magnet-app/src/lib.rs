extern crate hyper;
extern crate regex;
extern crate magnet_core;
extern crate magnet_more;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

mod app;
mod routing;
mod controller;

pub use app::{Application, DbPool};
pub use routing::RouteParams;
pub use controller::{Controller, Context, Response, respond};
pub use magnet_core::{Response as CoreResponse, Status, header};
pub use magnet_more::{QueryStringParams, Param};
pub use magnet_more::Value;
