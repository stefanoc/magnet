#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;
extern crate magnet_app;

use magnet_app::Application;

pub mod schema;
pub mod models;
pub mod controllers;

pub struct Error(String);

pub fn app() -> Application {
    let app = Application::new("example")
        .database(dotenv!("DATABASE_URL"))
        .view("companies/index",
              "<h2>Listing all companies</h2><hr><table>{% for company in companies \
               %}<tr><td>{{company.id}}</td><td><a \
               href=\"/companies/{{company.id}}\">{{company.name}}</a></td></tr>{% endfor \
               %}</table>")
        .view("companies/show",
              "<h2>Showing company \
               {{company.id}}</h2><p><dl><dt>Name:</dt><dd>{{company.name}}</dd><dt>Founded:\
               </dt><dd>{% if company.founded %}{{company.founded}}{% else %}N/A{% endif %}</p>")
        .route::<controllers::Companies>("companies", "/companies");
    app
}
