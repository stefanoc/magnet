use diesel::pg::data_types::PgDate;

#[derive(Queryable)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub founded: Option<PgDate>,
}
