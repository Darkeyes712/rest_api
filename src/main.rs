use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
    genre: String,
    published_date: String,
}

async fn create_mongo_client() -> Result<Client, Box<dyn Error>> {
    env::set_var("MONGODB_URI", "mongodb://localhost:27017");
    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    return Ok(client);
}

async fn create_book(book: web::Json<Book>) -> impl Responder {
    let book = book.into_inner();

    let mongo_client = create_mongo_client().await;

    // Insert the book into the collection
    match mongo_client {
        Ok(client) => match client
            .database("test")
            .collection::<Book>("books")
            .insert_one(&book, None)
            .await
        {
            Ok(_) => HttpResponse::Created().finish(),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// TODO:
// Add functions for GET, PUT and DELETE

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::scope("/api").route("/books", web::post().to(create_book)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
