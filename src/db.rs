use std::env;

#[tokio::main]
async fn db() {
    match env::var("POSTGRESQL_CONNECTION") {
        Ok(val) => println!("{}", val),
        Err(e) => println!("Error: {}", e)
    }
} 