use axum::{extract::State, Json};
use mongodb::Database;
use serde::Deserialize;

use crate::models::user::User;
use crate::services::user_service;

#[derive(Deserialize)]
pub struct AddUserRequest {
    pub name: String,
    pub email: String,
}

pub async fn get_users(State(db): State<Database>) -> Json<Vec<User>> {
    let users = user_service::get_all_users(&db).await.unwrap();
    Json(users)
}

pub async fn add_user(
    State(db): State<Database>,
    Json(payload): Json<AddUserRequest>,
) -> &'static str {
    user_service::add_user(&db, payload.name, payload.email)
        .await
        .unwrap();
    "User added successfully"
}
