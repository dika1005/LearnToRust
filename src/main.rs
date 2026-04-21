mod app_state;
mod entities;
mod dtos;
mod routes;
mod services;      // Tambahkan ini
mod repositories;  // Tambahkan ini

use actix_web::{web, App, HttpServer, Responder, get, HttpResponse};
use app_state::AppState;
use sea_orm::Database;
use std::env;
use dotenvy::dotenv;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Backend Bengkel API is Running!")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 
    
    // 1. Koneksi Database
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(db_url)
        .await
        .expect("Failed to connect to database");

    // 2. Inisialisasi State
    let state = web::Data::new(AppState { db });

    println!("🚀 Server running at http://127.0.0.1:8081");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            // Register Route Hello World
            .service(hello)
            // Grouping Route Auth (Register & Login)
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(routes::auth_routes::register))
                    // .route("/login", web::post().to(routes::auth_routes::login)) // Buka ini kalau login sudah siap
            )
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}