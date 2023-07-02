mod api;
mod models;
mod repository;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, Responder, Result, HttpServer};
use serde::{Serialize};

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    let response = Response {
        message: "Everything is working fine!".to_string(),
    };
    return HttpResponse::Ok().json(response);
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    return Ok(HttpResponse::NotFound().json(response));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let movie_db = repository::database::Database::new();
    let app_data = web::Data::new(movie_db);
    return HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(health_check)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
        .bind(("127.0.0.1", 4040))?
        .run()
        .await;
}
