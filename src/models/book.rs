
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize,FromRow)]
pub struct Book {
    pub id:String,
    pub title: String,
    pub author: String,
    pub pages: i32,
}

#[derive(Debug,Deserialize)]
pub struct CreateBookForm {
    pub title: String,
    pub author: String,
    pub pages: i32,
}

#[derive(Serialize)]
pub struct BooksList {
    pub books: Vec<Book>,
}