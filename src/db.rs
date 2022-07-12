use diesel::prelude::*;
use std::env;
use dotenv::dotenv;

pub fn establish_connection() -> SqliteConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
  
  SqliteConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}
