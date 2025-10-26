use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(&db_url).await.unwrap();
    println!("Database connected");
}
