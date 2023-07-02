use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Error};
use dotenv::dotenv;

use crate::models::movie::Movie;
use crate::repository::schema::movies::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn get_movies(&self) -> Vec<Movie> {
        movies
            .load::<Movie>(&mut self.pool.get().unwrap())
            .expect("Error loading all movies")
    }

    pub fn create_movie(&self, movie: Movie) -> Result<Movie, Error> {
        diesel::insert_into(movies)
            .values(&movie)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new movie");
        Ok(movie)
    }

    pub fn get_movie_by_id(&self, movie_id: i32) -> Option<Movie> {
        let movie = movies
            .find(movie_id)
            .get_result::<Movie>(&mut self.pool.get().unwrap())
            .expect("Error loading todo by id");
        Some(movie)
    }

    pub fn delete_movie_by_id(&self, movie_id: i32) -> Option<usize> {
        let count = diesel::delete(movies.find(movie_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting todo by id");
        Some(count)
    }

    pub fn update_movie_by_id(&self, movie_id: i32, movie: Movie) -> Option<Movie> {
        let conn = &mut self.pool.get().unwrap();
        let movie = diesel::update(movies.find(movie_id))
            .set(&movie)
            .execute(conn)
            .expect("Error updating movie by id");
        movies.find(movie_id).first(conn).ok()
    }
}