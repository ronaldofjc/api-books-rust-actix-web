use std::env;
use mongodb::{Client, Collection, options};
use crate::entity::book::Book;
use dotenv::dotenv;
use futures::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::bson::extjson::de::Error;
use mongodb::bson::oid::ObjectId;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};

pub struct MongoRepo {
    col: Collection<Book>
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database("api");
        let col: Collection<Book> = db.collection("Book");
        MongoRepo { col }
    }

    pub async fn create_book(&self, new_book: Book) -> Result<InsertOneResult, Error> {
        let book = self
            .col
            .insert_one(new_book, None)
            .await
            .ok()
            .expect("Ocorreu um erro ao criar um Livro");

        Ok(book)
    }

    pub async fn get_book_by_id(&self, id: &String) -> Result<Book, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let book = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Ocorreu um erro ao recuperar um livro");

        Ok(book.unwrap())
    }

    pub async fn update(&self, id: String, book: Document) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let book = self
            .col
            .update_one(filter, book, None)
            .await
            .ok()
            .expect("Ocorreu um erro ao atualizar um livro");

        Ok(book)
    }

    pub async fn find_all(&self) -> Result<Vec<Book>, Error> {
        let filter = doc! { "active": true };
        let options = options::FindOptions::builder().sort(doc! { "title": 1 }).build();
        let mut cursor = self.col.find(filter, options).await.ok().expect("error");
        let mut books = Vec::new();
        while let Ok(Some(book)) = cursor.try_next().await {
            books.push(book);
        }

        Ok(books)
    }

    pub async fn delete_by_id(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let book = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Ocorreu um erro ao remover um livro");

        Ok(book)
    }
}