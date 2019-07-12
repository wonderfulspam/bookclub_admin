#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer, Responder, HttpRequest};
use diesel::prelude::*;
use diesel::SqliteConnection;
use serde::{Serialize};
use dotenv::dotenv;
use std::env;
use schema::books;
use models::*;

mod models;
mod schema;

#[derive(Serialize)]
struct BookList {
    books: Vec<Book>,
}

fn list_books() -> impl Responder {
    use self::schema::books::dsl::*;
    let connection = establish_connection();
    let results = books.load::<Book>(&connection).expect("Failed to load books");

    web::Json(BookList { books: results})
}

fn upload() -> impl Responder {

}

fn get_book(req: HttpRequest) -> impl Responder {
    let book_id = req.match_info().get("id").unwrap();
    format!("Here is book {}", &book_id)
}

fn index() -> impl Responder {

}

fn test_add_book() -> impl Responder {
    let connection = establish_connection();
    let title = String::from("Bogtitel");
    let mime_type = String::from("En mime type");
    let content = vec![1, 2, 10];
    let new_book = NewBook {
        title: &title,
        mime_type: &mime_type,
        content: &content
    };

    diesel::insert_into(books::table).values(&new_book).execute(&connection).expect("Failed to insert");
    format!("We did it! Inserted {}", &title)
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Could not find db url");
    SqliteConnection::establish(&database_url).expect("Could not connect to db")
}

fn main() {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/books", web::get().to(list_books))
            .route("/books", web::post().to(upload))
            .route("/books/{id}", web::get().to(get_book))
            .route("/test", web::get().to(test_add_book))
    }).bind("127.0.0.1:8123").unwrap().run().unwrap();
}
