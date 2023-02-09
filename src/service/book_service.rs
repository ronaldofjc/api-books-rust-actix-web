use actix_web::web::{Data, Json, Path};
use chrono::Utc;
use mongodb::bson::{doc, Document};
use mongodb::bson::oid::ObjectId;
use serde_json::{json, Value};
use crate::entity::book::Book;
use crate::entity::create_book::CreateBook;
use crate::entity::error_data::ErrorData;
use crate::MongoRepo;
use crate::entity::update_book::UpdateBook;

pub struct BookService {}

impl BookService {
    pub async fn create(payload: Json<CreateBook>, db: Data<MongoRepo>) -> Result<ObjectId, ErrorData> {
        if has_invalid_params(payload.title.clone(), payload.author.clone(), payload.pages.clone()) {
            return Err(ErrorData::new("Parametros inválidos".to_string(), 400));
        }

        let book = Book {
            id: None,
            title: payload.title.clone().unwrap(),
            author: payload.author.clone().unwrap(),
            pages: payload.pages.clone().unwrap(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            active: true
        };

        let new_book = db.create_book(book).await;
        match new_book {
            Ok(result) => {
                tracing::info!("Livro {} foi criado com sucesso", payload.title.clone().unwrap());
                Ok(result.inserted_id.as_object_id().unwrap())
            },
            Err(err) => Err(ErrorData::new(err.to_string(), 502))
        }
    }

    pub async fn get_by_id(path: Path<String>, db:Data<MongoRepo>) -> Result<Book, ErrorData> {
        let id = path.into_inner();
        if id.is_empty() {
            return Err(ErrorData::new("ID Inválido".to_string(), 400));
        }

        let book = db.get_book_by_id(&id).await;
        match book {
            Ok(b) => Ok(b),
            Err(err) => Err(ErrorData::new(err.to_string(), 502))
        }
    }

    pub async fn update(path: Path<String>, update_book: Json<UpdateBook>, db:Data<MongoRepo>) -> Result<Book, ErrorData> {
        if has_invalid_params(update_book.title.clone(), update_book.author.clone(), update_book.pages.clone()) {
            return Err(ErrorData::new("Parametros inválidos".to_string(), 400));
        }

        let id = path.into_inner();
        if id.is_empty() {
            return Err(ErrorData::new("ID Inválido".to_string(), 400));
        }

        let res = db.get_book_by_id(&id).await;
        match res {
            Ok(book) => {
                let new_doc = create_new_document(&update_book, &book, id.clone());
                match db.update(id.clone(), new_doc).await {
                    Ok(updated) => {
                        if updated.matched_count == 1 {
                            match db.get_book_by_id(&id).await {
                                Ok(b) => Ok(b),
                                Err(err) => Err(ErrorData::new(err.to_string(), 502))
                            }
                        } else {
                            return Err(ErrorData::new("Livro nao encontrado pelo id informado".to_string(), 400))
                        }
                    },
                    Err(err) => Err(ErrorData::new(err.to_string(), 502))
                }
            },
            Err(err) => Err(ErrorData::new(err.to_string(), 502))
        }
    }

    pub async fn find_all_actives(db:Data<MongoRepo>) -> Result<Vec<Book>, ErrorData> {
        match db.find_all().await {
            Ok(books) => Ok(books),
            Err(err) => Err(ErrorData::new(err.to_string(), 502))
        }
    }

    pub async fn delete_by_id(path: Path<String>, db:Data<MongoRepo>) -> Result<Json<Value>, ErrorData> {
        let id = path.into_inner();
        if id.is_empty() {
            return Err(ErrorData::new("ID Inválido".to_string(), 400));
        }

        match db.delete_by_id(&id).await {
            Ok(res) => {
                if res.deleted_count == 1 {
                    Ok(Json(json!({ "message":"Livro removido com sucesso"})))
                } else {
                    Err(ErrorData::new("Ocorreu um erro ao remover o livro".to_string(), 502))
                }
            },
            Err(err) => Err(ErrorData::new(err.to_string(), 502))
        }
    }
}

fn create_new_document(update_book: &UpdateBook, book: &Book, path: String) -> Document {
    doc! {
        "$set": {
            "_id": Some(ObjectId::parse_str(&path).unwrap()),
            "title": update_book.title.as_ref().unwrap().clone(),
            "author": update_book.author.as_ref().unwrap().clone(),
            "pages": update_book.pages.as_ref().unwrap().clone(),
            "created_at": book.created_at.clone(),
            "updated_at": Utc::now(),
            "active": true
        }
    }
}

fn has_invalid_params(title: Option<String>, author: Option<String>, pages: Option<i64>) -> bool {
    if title.is_none() || author.is_none() || pages.is_none() { return true }  return false
}