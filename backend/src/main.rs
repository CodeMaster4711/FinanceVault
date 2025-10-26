use dotenvy::dotenv;
use sea_orm::{Database};
use std::env;
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(&db_url).await.unwrap();
    println!("Database connected");

    Migrator::up(&conn, None).await.unwrap();
    println!("Migrations applied");
}
