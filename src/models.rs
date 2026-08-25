use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::cities)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct City {
    pub id: i32,
    pub name: String,
}
