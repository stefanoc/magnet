use regex::Regex;
use std::collections::HashMap;
use magnet_more::params::{Param, NULL_PARAM};

#[derive(Debug)]
pub struct Route {
    pub name: String,
    pub definition: String,
    pub pattern: Regex,
}

#[derive(Debug)]
pub struct RouteParams {
    params: HashMap<String, Param>,
}

impl Route {
    fn parse(def: &str) -> String {
        let param_expr = Regex::new(r":(?P<k>[a-zA-Z_]+)").unwrap();
        let route_expr = param_expr.replace_all(def, "(?P<$k>[^/]*)");
        format!("\\A{}\\z", route_expr)
    }

    pub fn new(name: String, def: &str) -> Route {
        if let Ok(pattern) = Regex::new(&Route::parse(def)) {
            Route {
                name: name,
                definition: def.into(),
                pattern: pattern,
            }
        } else {
            panic!("Invalid route: {}", def);
        }
    }

    pub fn matches(&self, path: &str) -> Option<RouteParams> {
        if let Some(captures) = self.pattern.captures(path) {
            let mut params = RouteParams::new();
            for (key, value) in captures.iter_named() {
                params.set(key, value.unwrap_or(""));
            }
            Some(params)
        } else {
            None
        }
    }
}

impl RouteParams {
    fn new() -> RouteParams {
        RouteParams { params: HashMap::new() }
    }

    fn set(&mut self, key: &str, value: &str) {
        self.params.insert(key.into(), Param::Single(value.into()));
    }

    pub fn get(&self, key: &str) -> &Param {
        self.params.get(key.into()).unwrap_or(&NULL_PARAM)
    }
}
