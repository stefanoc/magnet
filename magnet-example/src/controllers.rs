use magnet_app::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::collections::HashMap;
use super::models::*;

pub struct Companies;

impl Controller for Companies {
    fn index(context: Context) -> Response {
        let db = context.db().get().unwrap();
        let mut view = context.view();
        view.set("companies",
                 Value::Array(Self::all_companies(&db).iter().map(Companies::to_liquid).collect()));
        respond::render(Status::Ok, view)
    }

    fn show(context: Context) -> Response {
        let company_id = context.route_param("id").to_int().unwrap();
        let db = context.db().get().unwrap();
        if let Some(company) = Self::find_company(company_id, &db) {
            let mut view = context.view();
            view.set("company", Companies::to_liquid(&company));
            respond::render(Status::Ok, view)
        } else {
            respond::error(Status::NotFound, "Not found")
        }
    }
}

impl Companies {
    fn to_liquid(company: &Company) -> Value {
        use magnet_app::Value::*;
        let mut h = HashMap::new();
        h.insert("id".into(), Num(company.id as f32));
        h.insert("name".into(), Str(company.name.clone()));
        if let Some(ref founded) = company.founded {
            h.insert("founded".into(), Str(format!("{:?}", founded)));
        }
        Object(h)
    }

    fn all_companies(db: &PgConnection) -> Vec<Company> {
        use super::schema::companies::dsl::*;

        let all_companies = companies.load::<Company>(db)
            .expect("Failed to load companies");
        all_companies
    }

    fn find_company(company_id: i32, db: &PgConnection) -> Option<Company> {
        use super::schema::companies::dsl::*;
        companies.find(company_id).first(db).ok()
    }
}
