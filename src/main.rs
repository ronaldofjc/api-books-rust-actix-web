mod entrypoint;
mod entity;
mod service;
mod repository;

use actix_web::{App, HttpServer};
use actix_web::web::Data;
use crate::entrypoint::routes::{create_book, health, it_works, get_book_by_id, update_book_by_id, find_all, delete_book_by_id};
use crate::repository::mongo_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("8090".to_string());
    let address = format!("127.0.0.1:{}", port);

    tracing::info!("Starting server on {}", address);

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(it_works)
            .service(health)
            .service(create_book)
            .service(get_book_by_id)
            .service(update_book_by_id)
            .service(find_all)
            .service(delete_book_by_id)
    })
        .bind(&address)
        .unwrap_or_else(|err| {
            panic!("🔥🔥🔥 Couldn't start the server in port {}: {:?}", port, err)
        })
        .run()
        .await
}
