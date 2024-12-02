use sqlx::FromRow; // Needed to derive `FromRow` for the struct
use serde::{Deserialize, Serialize};
use uuid::Uuid; 

#[derive(Debug, FromRow,Serialize,Deserialize)] // Automatically maps database rows to the struct
pub struct User{
    pub id:String,
    pub first_name: String,
    pub second_name: String,
    pub email: String,
    pub dob: String,
}


#[derive(Serialize)]
pub struct UserList{
    pub users: Vec<User>,
    }

#[derive(Deserialize)]
pub struct CreateUserForm {
    pub first_name: String,
    pub second_name: String,
    pub email: String,
    pub dob: String,  // or chrono::NaiveDate depending on your setup
    
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user: User,
}


