mod controllers;
mod models;
use axum::{routing::{get,post}, Router};
use sqlx::mysql::MySqlPool;
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid; 
 // Import Server from hyper
 use tera::{Tera, Context};




 fn main() {
 connect_db();

}


#[tokio::main]
async fn connect_db() -> Result<(), sqlx::Error> {
    let database_url = "mysql://root:load@localhost:3306/rust";

    // Try to create a connection pool
    let pool = match MySqlPool::connect(database_url).await {
        Ok(pool) => {
            println!("Successfully connected to the database.");
            pool
        }
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            return Err(e); // Exit with an error
        }
    };

    // Clone the pool for sharing across handlers
    let shared_pool = Arc::new(pool);
     // Load Tera templates
     let tera = Arc::new(Tera::new("templates/**/*.html").unwrap());


    // Build the Axum app with routes
    let router = Router::new()
        .route("/get/users", get(controllers::userscontroller::get_users))
        .route("/post/user", post(controllers::userscontroller::save_user))
        .route("/get/user/:id", get(controllers::userscontroller::get_user_by_id)) 
        .with_state(shared_pool);

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum_server::bind(addr)
    .serve(router.into_make_service())
    .await
    .unwrap();

    Ok(())
}
