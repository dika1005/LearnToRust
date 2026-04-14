use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hallo njir")
}

async fn new_hello() -> impl Responder {
    HttpResponse::Ok().body("hallo njir1")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/hallo", web::get().to(new_hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}