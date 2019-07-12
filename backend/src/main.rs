use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};

fn list_books() -> impl Responder {
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Køb nu bare den fucking bog, for fanden.")
}

fn upload() -> impl Responder {

}

fn get_book(req: HttpRequest) -> impl Responder {
    let book_id = req.match_info().get("id").unwrap();
    format!("Here is book {}", &book_id)
}

fn index() -> impl Responder {

}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/books", web::get().to(list_books))
            .route("/books", web::post().to(upload))
            .route("/books/{id}", web::get().to(get_book))
    }).bind("127.0.0.1:8123").unwrap().run().unwrap();
}
