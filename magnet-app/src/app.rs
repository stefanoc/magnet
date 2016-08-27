#![allow(unused_imports)]

use magnet_core::{Stack, Request, Responder, Before};
use magnet_core::ext::Key as ExtKey;
use magnet_more::params::{QueryStringParser, QueryStringParams};
use magnet_more::views::{Templates, Views, View};
use super::routing::{Route, RouteParams};
use super::controller::{Controller, Dispatcher};
use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;

pub struct Application {
    name: &'static str,
    templates: Templates,
    database_url: Option<String>,
    root: Stack,
}

pub type DieselPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbPool;

impl ExtKey for DbPool {
    type Value = DieselPool;
}

impl Application {
    pub fn new(name: &'static str) -> Application {
        Application {
            name: name,
            templates: Templates::new(),
            database_url: None,
            root: Stack::new(),
        }
    }

    pub fn root(self) -> Stack {
        self.root
    }

    pub fn database<T>(mut self, url: T) -> Application
        where T: Into<String>
    {
        self.database_url = Some(url.into());
        self
    }

    pub fn mount(mut self, app: Application) -> Application {
        self.root.mount(app.root());
        self
    }

    pub fn view<T>(mut self, name: &str, source: T) -> Application
        where T: Into<String>
    {
        self.templates.insert(name, source);
        self
    }

    pub fn route<T>(mut self, name: &'static str, path: &'static str) -> Application
        where T: Controller
    {
        let dispatcher = Dispatcher::<T>::new(name, path);
        self.root.add(dispatcher);
        self

    }

    pub fn run(mut self) {
        use hyper::server::Server;

        self.root.set::<Views>(self.templates);
        if let Some(db_url) = self.database_url {
            let config = r2d2::Config::default();
            let manager = ConnectionManager::<PgConnection>::new(db_url);
            let pool = r2d2::Pool::new(config, manager).expect("Failed to create pool.");
            self.root.set::<DbPool>(pool);
        }
        self.root.before(QueryStringParser);
        Server::http("0.0.0.0:3000").unwrap().handle(self.root).unwrap();
    }
}
