use std::env;

use sea_orm::{Database, DatabaseConnection};

pub async fn init_db() -> DatabaseConnection {
    println!("DATABASE_URL: {}", env::var("DATABASE_URL").expect("DATABASE_URL must be set"));
    match Database::connect(
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set")
            .as_str(),
    )
    .await
    {
        Ok(db) => db,
        Err(e) => {
            panic!("Error connecting to database: {}", e);
        }
    }
}