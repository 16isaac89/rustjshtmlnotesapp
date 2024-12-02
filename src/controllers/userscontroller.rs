use crate::models;
use sqlx::mysql::MySqlPool;
use serde::Serialize;
use axum::{extract::{State, Path},Json,http::StatusCode,Form};
use std::sync::Arc;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};


impl models::user::User {
    pub fn full_name(&self) -> String {
        let fullname = format!("{} {}", self.first_name, self.second_name);
        fullname
    }
    pub fn age(&self) -> String {
     format!("{}",self.dob)
    }
}


// Define the get_users function
pub async fn get_users(
    State(pool): State<Arc<MySqlPool>>, // Use `State` to extract shared state
) -> Result<Json<models::user::UserList>, (StatusCode, String)> {
    match sqlx::query_as::<_, models::user::User>("SELECT * FROM users")
        .fetch_all(&*pool)
        .await
    {
        Ok(users) => Ok(Json(models::user::UserList { users })),
        Err(e) => {
            eprintln!("Failed to fetch users: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch users".to_string(),
            ))
        }
    }
}

pub async fn save_user(
    State(pool): State<Arc<MySqlPool>>,
    Form(new_user): Form<models::user::CreateUserForm>,
) -> Json<&'static str>{
    //TODO: Implement save_user function
    let query = r#"
        INSERT INTO users (id,password,first_name, second_name, email, dob)
        VALUES (?,?,?, ?, ?, ?)
    "#;
    let id = Uuid::new_v4().to_string();
    let activestate = 1;
    let password_hash = match hash(new_user.password.as_bytes(), DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return Json("Failed to hash password.")
    };
    match sqlx::query(query)
        .bind(id)
       
        .bind(password_hash)
        .bind(new_user.first_name)
        .bind(new_user.second_name)
        .bind(new_user.email)
        .bind(new_user.dob)
        .execute(&*pool)
        .await
    {
        Ok(_) => Json("User saved successfully."),
        Err(err) => {
            eprintln!("Failed to save user: {}", err);
            Json("Failed to save user.")
        }
    }

}

pub async fn get_user_by_id(
    State(pool): State<Arc<MySqlPool>>, // Access shared pool
    Path(id): Path<String>, 
)-> Result<Json<models::user::UserResponse>, (StatusCode, String)>{
    // Query the database for a user by ID
    match sqlx::query_as::<_, models::user::User>("SELECT * FROM users WHERE id = ?")
        .bind(&id) // Bind the ID parameter
        .fetch_one(&*pool) // Fetch a single result
        .await
    {
        Ok(user) => Ok(Json(models::user::UserResponse { user })),
        Err(e) => {
            eprintln!("Failed to fetch user by ID: {}", e);
            Err((
                StatusCode::NOT_FOUND,
                format!("User with ID {} not found", id),
            ))
        }
    }

}