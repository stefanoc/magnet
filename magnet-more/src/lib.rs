#![allow(dead_code)]

extern crate magnet_core;
extern crate regex;
extern crate liquid;

pub mod params;
pub mod views;

pub use params::{QueryStringParams, Params, Param, QueryStringParser};
pub use views::{Views, Templates, View, Value};
