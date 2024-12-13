use std::sync::Arc;
use sqlx::{MySql, MySqlPool};
use axum::{extract::{Path, State}, http::StatusCode, response::{Html, Form, Json}};
use uuid::Uuid;
use crate::models;

pub async fn save_book(
    State(pool):State<Arc<MySqlPool>>,
    Form(create_book):Form<models::book::CreateBookForm>
)->Json<&'static str>{
let query = r#"INSERT INTO books (id,title,author,pages)
                    VALUES (?,?,?,?)
                    "#;
let id = Uuid::new_v4().to_string();
match sqlx::query(query)
            .bind(id)
            .bind(create_book.title)
            .bind(create_book.author)
            .bind(create_book.pages)
            .execute(&*pool)
            .await
            {
                Ok(_)=>Json("Book saved successfully"),
                Err(err)=>{
                    eprintln!("Failed to save book {}",err);
                    Json("Failed to save the book")
                }
            }
}

pub async fn get_books(
    State(pool):State<Arc<MySqlPool>>

)->Result<Json<models::book::BooksList>,(StatusCode,String)>{
    match::sqlx::query_as::<_, models::book::Book>("SELECT * FROM books")
    .fetch_all(&*pool)
    .await{
        Ok(books) => Ok(Json(models::book::BooksList{books})),
        Err(err) =>{
            eprintln!("Failed to get books {}",err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch books".to_string()
            ))
        }
    }
}