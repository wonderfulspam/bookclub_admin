use super::schema::books;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Book {
    pub id: i32,
    pub title: String,
    #[serde(skip_serializing)]
    pub content: Vec<u8>,
    pub mime_type: String
}

#[derive(Insertable)]
#[table_name="books"]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub mime_type: &'a str,
    pub content: &'a Vec<u8>,
}