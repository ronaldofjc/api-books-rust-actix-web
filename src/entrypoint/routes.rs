use actix_web::{HttpResponse, get, post, put, delete};
use actix_web::web::{Data, Json, Path};
use serde_json::json;
use crate::entity::create_book::CreateBook;
use crate::entity::heath::Health;
use crate::entity::update_book::UpdateBook;
use crate::MongoRepo;
use crate::service::book_service::BookService;

#[get("/")]
pub async fn it_works() -> HttpResponse {
    HttpResponse::Ok().json(Json(json!({ "message":"API Rust with Actix Web Works!!!"})))
}

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(Health { status: "Ok".to_string() })
}

#[post("/books")]
pub async fn create_book(payload: Json<CreateBook>, db: Data<MongoRepo>) -> HttpResponse {
    match BookService::create(payload, db).await {
        Ok(id) => HttpResponse::Created().json(Json(json!({"id": id}))),
        Err(error) => HttpResponse::BadRequest().json(error)
    }
}

#[get("/books/{id}")]
pub async fn get_book_by_id(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    match BookService::get_by_id(path, db).await {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(error) => HttpResponse::BadRequest().json(error)
    }
}

#[put("/books/{id}")]
pub async fn update_book_by_id(db: Data<MongoRepo>, path: Path<String>, update_book: Json<UpdateBook>) -> HttpResponse {
    match BookService::update(path, update_book, db).await {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(error) => HttpResponse::BadRequest().json(error)
    }
}

#[get("/books")]
pub async fn find_all(db: Data<MongoRepo>) -> HttpResponse {
    match BookService::find_all_actives(db).await {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(error) => HttpResponse::BadRequest().json(error)
    }
}

#[delete("/books/{id}")]
pub async fn delete_book_by_id(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    match BookService::delete_by_id(path, db).await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(error) => HttpResponse::BadRequest().json(error)
    }
}