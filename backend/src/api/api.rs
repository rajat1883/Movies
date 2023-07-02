use actix_web::web;
use actix_web::{web::{
    Data,
    Json,
}, post, get, delete, put, HttpResponse};
use crate::{models::movie::Movie, repository::database::Database};


#[post("/movies")]
pub async fn create_movie(db: Data<Database>, new_movie: Json<Movie>) -> HttpResponse {
    let movie = db.create_movie(new_movie.into_inner());
    match movie {
        Ok(movie) => HttpResponse::Ok().json(movie),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/movies/{id}")]
pub async fn get_movie_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let movie = db.get_movie_by_id(id.parse().unwrap());

    match movie {
        Some(movie) => HttpResponse::Ok().json(movie),
        None => HttpResponse::NotFound().body("Movie not found"),
    }
}

#[get("/movies")]
pub async fn get_movies(db: web::Data<Database>) -> HttpResponse {
    let movies = db.get_movies();
    HttpResponse::Ok().json(movies)
}

#[delete("/movies/{id}")]
pub async fn delete_movie_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let movie = db.delete_movie_by_id(id.parse().unwrap());
    match movie {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Movie not found"),
    }
}

#[put("/movies/{id}")]
pub async fn update_movie_by_id(db: web::Data<Database>, id: web::Path<String>, updated_movie: web::Json<Movie>) -> HttpResponse {
    let movie = db.update_movie_by_id(id.parse().unwrap(), updated_movie.into_inner());
    match movie {
        Some(movie) => HttpResponse::Ok().json(movie),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_movie)
            .service(get_movie_by_id)
            .service(get_movies)
            .service(delete_movie_by_id)
            .service(update_movie_by_id)
    );
}
