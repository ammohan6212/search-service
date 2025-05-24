mod routes;

use actix_web::{App, HttpServer};
use routes::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .configure(config) // Load route configuration
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
