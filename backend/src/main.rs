use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Køb nu bare den fucking bog, for fanden.")
}

fn main() {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    }).bind("127.0.0.1:8123").unwrap().run().unwrap();
}
