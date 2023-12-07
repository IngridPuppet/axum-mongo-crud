mod controller;
mod model;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use mongodb::{error::Error as MongoError, options::ClientOptions, Client, Database};
use std::net::SocketAddr;
use tokio::net::TcpListener;

fn app(db: Database) -> Router {
    Router::new()
        .route("/books", get(controller::book::fetch_all))
        .route("/books/:id", get(controller::book::fetch_one))
        .route("/books", post(controller::book::store))
        .route("/books/:id", put(controller::book::update))
        .route("/books/:id", delete(controller::book::delete))
        .with_state(db)
}

#[tokio::main]
async fn main() {
    // Load dotenv variables
    dotenv::dotenv().ok();

    // Set up MongoDB connection
    let db = config_mongo().await.expect("failed to connect to database");

    // Set up server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("server listening on {}", addr);

    // Serve app
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app(db)).await.unwrap();
}

async fn config_mongo() -> Result<Database, MongoError> {
    // Read config vars
    let mongo_url = std::env::var("MONGO_URI").expect("missing `MONGO_URI` config");
    let mongo_dbn = std::env::var("MONGO_DBN").expect("missing `MONGO_DBN` config");

    // Parse a connection string into an options struct.
    let client_options = ClientOptions::parse(&mongo_url).await?;

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // Get a handle to a database.
    Ok(client.database(&mongo_dbn))
}
