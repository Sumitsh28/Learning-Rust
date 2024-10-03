use mongodb::{Client, options::ClientOptions};
use std::env;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get MongoDB URI from environment variable
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // Parse the connection string into `ClientOptions`
    let options = ClientOptions::parse(&client_uri).await?;
    
    // Initialize the MongoDB client
    let client = Client::with_options(options)?;

    // List and print database names
    println!("Databases:");
    for name in client.list_database_names().await? {
        println!("- {}", name);
    }

    Ok(())
}
