use std::env;

use sea_orm::{Database, DatabaseConnection};

pub async fn init_db() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("DATABASE_URL: {}", db_url);
    match Database::connect(db_url.as_str()).await {
        Ok(db) => db,
        Err(e) => {
            panic!("Error connecting to database: {}", e);
        }
    }
}