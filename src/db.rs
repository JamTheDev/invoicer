
use std::env;

use sqlx::postgres::{
    PgPoolOptions,
};


pub async fn initialize_database() {
    match env::var("POSTGRESQL_CONNECTION") {
        Ok(val) => connect(val).await,
        Err(e) => println!("Error: {}", e)
    };
} 

// i know this is redundant but yeah ill just work on this in another time
async fn connect(val: String) {
    let _ = PgPoolOptions::new()
        .max_connections(5)
        .connect(&val).await;
}