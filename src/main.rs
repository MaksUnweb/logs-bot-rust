use crate::includes::start_bot;
mod includes;
use sqlx::postgres::{PgPool, PgPoolOptions};
use dotenv::dotenv;
use std::env;


async fn connection_to_db(url: String) -> Result<PgPool, sqlx::error::Error> {
  let pool = PgPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(std::time::Duration::from_secs(5))
    .connect(&url)
    .await?;
  
  Ok(pool)
}


#[tokio::main]
async fn main() {
  dotenv().ok();
  let db_url = env::var("DB_URL").expect("Database connection error! Couldn't extract URL!");
  let pool = match connection_to_db(db_url).await {
    Ok(pool) => { pool },
    Err(e) => { panic!("{}", e) }
  };
  let _ = start_bot(pool).await;
}
