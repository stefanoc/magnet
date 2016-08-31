use magnet_core::{Stack, Responder, Request, Response as CoreResponse, Status, Method,
                  MagnetResult};
use magnet_more::{Views, View};
use magnet_more::QueryStringParams;
use magnet_more::params::{Param, NULL_PARAM};
use super::routing::{RouteParams, Route};
use super::DbPool;
use super::app::DieselPool;
use std::marker::PhantomData;

pub struct Context<'a> {
    pub stack: &'a Stack,
    pub request: &'a Request,
    pub route_params: RouteParams,
    pub template_name: String,
}

pub enum Response {
    Bare(CoreResponse),
    Redirect(Status, String),
    Render(Status, View),
    Error(Status, String),
}

impl Response {
    fn build(self) -> CoreResponse {
        use magnet_core::header;

        match self {
            Response::Bare(response) => response,
            Response::Redirect(status, url) => {
                CoreResponse::build(status).header(header::Location(url)).end()
            }
            Response::Render(status, mut view) => {
                CoreResponse::build(status).html(view.render()).end()
            }
            Response::Error(status, body) => CoreResponse::build(status).body(body).end(),
        }
    }
}

pub mod respond {
    use super::Response;
    use ::magnet_core::{Response as CoreResponse, Status};
    use ::magnet_more::View;

    pub fn with(response: CoreResponse) -> Response {
        Response::Bare(response)
    }

    pub fn redirect<T>(url: T) -> Response
        where T: Into<String>
    {
        Response::Redirect(Status::TemporaryRedirect, url.into())
    }

    pub fn render(status: Status, view: View) -> Response {
        Response::Render(status, view)
    }

    pub fn error<T>(status: Status, body: T) -> Response
        where T: Into<String>
    {
        Response::Error(status, body.into())
    }
}

pub trait Controller: Sync + Send + 'static {
    fn index(_context: Context) -> Response {
        respond::error(Status::NotImplemented, "")
    }

    fn show(_context: Context) -> Response {
        respond::error(Status::NotImplemented, "")
    }

    fn new(_context: Context) -> Response {
        respond::error(Status::NotImplemented, "")
    }

    fn create(_context: Context) -> Response {
        respond::error(Status::NotImplemented, "")
    }

    fn edit(_context: Context) -> Response {
        respond::error(Status::NotImplemented, "")
    }

    fn update(_context: Context) -> Response {
        respond::error(Status::NotImplemented, "")
    }

    fn destroy(_context: Context) -> Response {
        respond::error(Status::NotImplemented, "")
    }
}

pub struct Dispatcher<T> {
    name: &'static str,
    root: &'static str,
    map: Vec<(&'static str, Method, Route)>,
    _controller: PhantomData<T>,
}

impl<'a> Context<'a> {
    pub fn view(&self) -> View {
        let lib = self.stack.get::<Views>().unwrap();
        lib.view(&self.template_name)
    }

    pub fn route_param(&self, key: &str) -> &Param {
        self.route_params.get(key)
    }

    pub fn query_param(&self, key: &str) -> &Param {
        if let Some(params) = self.request.get::<QueryStringParams>() {
            params.get(key)
        } else {
            &NULL_PARAM
        }
    }

    pub fn db(&self) -> &DieselPool {
        if let Some(pool) = self.stack.get::<DbPool>() {
            pool
        } else {
            panic!("Database not available");
        }
    }
}

impl<T> Dispatcher<T>
    where T: Controller
{
    pub fn new(name: &'static str, root: &'static str) -> Dispatcher<T> {
        let mut map = vec![];
        map.push(("index", Method::Get, Route::new(root)));
        map.push(("new", Method::Get, Route::new(&format!("{}/new", root))));
        map.push(("create", Method::Post, Route::new(root)));
        map.push(("show", Method::Get, Route::new(&format!("{}/:id", root))));
        map.push(("edit", Method::Get, Route::new(&format!("{}/:id/edit", root))));
        map.push(("update", Method::Put, Route::new(&format!("{}/:id", root))));
        map.push(("destroy", Method::Delete, Route::new(&format!("{}/:id", root))));

        Dispatcher {
            name: name,
            root: root,
            map: map,
            _controller: PhantomData,
        }
    }
}

impl<T> Responder for Dispatcher<T>
    where T: Controller
{
    fn call(&self, stack: &Stack, request: &Request) -> MagnetResult<Option<CoreResponse>> {
        for item in self.map.iter().filter(|item| item.1 == request.method) {
            if let Some(params) = item.2.matches(request.path()) {
                let context = Context {
                    template_name: format!("{}/{}", self.name, item.0),
                    stack: stack,
                    request: request,
                    route: &item.2,
                    route_params: params,
                };
                let response = T::before(&context)
                    .unwrap_or(Dispatcher::<T>::invoke(item.0, context));
                return Ok(Some(response.build()));
            }
        }
        Ok(None)
    }
}

impl<T> Dispatcher<T>
    where T: Controller
{
    fn invoke(action: &str, context: Context) -> Response {
        match action {
            "index" => T::index(context),
            "new" => T::new(context),
            "create" => T::create(context),
            "show" => T::show(context),
            "edit" => T::edit(context),
            "update" => T::update(context),
            "destroy" => T::destroy(context),
            _ => panic!("implement me"),
        }
    }
}
