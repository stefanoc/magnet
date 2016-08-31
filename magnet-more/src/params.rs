use regex::Regex;
use std::collections::HashMap;
use magnet_core::{Stack, Before, Request, MagnetResult};
use magnet_core::ext::Key;

#[derive(Debug,Clone)]
pub struct Params {
    data: HashMap<String, Param>,
}

#[derive(Debug,Clone)]
pub enum Param {
    None,
    Flag,
    Single(String),
}

pub struct QueryStringParser;

pub struct QueryStringParams;

pub static NULL_PARAM: Param = Param::None;

impl Param {
    pub fn single(&self) -> Option<&String> {
        match *self {
            Param::Single(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn flag(&self) -> bool {
        match *self {
            Param::Flag => true,
            _ => false,
        }
    }

    pub fn to_int(&self) -> Option<i32> {
        if let Some(val) = self.single() {
            val.parse::<i32>().ok()
        } else {
            None
        }
    }
}

impl Params {
    pub fn get(&self, key: &str) -> &Param {
        self.data.get(key).unwrap_or(&NULL_PARAM)
    }
}

impl Before for QueryStringParser {
    fn call(&self, _stack: &Stack, request: &mut Request) -> MagnetResult<()> {
        if let Some(params) = {
            request.query().map(|qs| parse(qs))
        } {
            request.set::<QueryStringParams>(params);
        }
        Ok(())
    }
}

impl Key for QueryStringParams {
    type Value = Params;
}

fn parse(qs: &str) -> Params {
    let mut result = Params { data: HashMap::new() };
    let _key_expr = Regex::new(r"(.+?)(\[.*?\])?").unwrap();

    for s in qs.split("&") {
        let mut p = s.split("=");
        let full_key = p.nth(0).unwrap().into();

        match p.nth(0) {
            Some(v) => result.data.insert(full_key, Param::Single(v.into())),
            None => result.data.insert(full_key, Param::Flag),
        };
    }
    result
}
