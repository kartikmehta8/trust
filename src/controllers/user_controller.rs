use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use mongodb::Database;
use serde::Deserialize;

use crate::services::user_service;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

fn error_response(message: &str) -> Response {
    (axum::http::StatusCode::BAD_REQUEST, message.to_string()).into_response()
}

pub async fn sign_up(State(db): State<Database>, Json(payload): Json<AuthRequest>) -> Response {
    match user_service::sign_up(&db, payload.email, payload.password).await {
        Ok(_) => (axum::http::StatusCode::OK, "User registered successfully").into_response(),
        Err(err) => error_response(&err),
    }
}

pub async fn login(State(db): State<Database>, Json(payload): Json<AuthRequest>) -> Response {
    match user_service::login(&db, payload.email, payload.password).await {
        Ok(token) => Json(token).into_response(),
        Err(err) => error_response(&err),
    }
}

pub async fn forgot_password(
    State(db): State<Database>,
    Json(payload): Json<ForgotPasswordRequest>,
) -> Response {
    match user_service::forgot_password(&db, payload.email).await {
        Ok(_) => (axum::http::StatusCode::OK, "Password reset code sent").into_response(),
        Err(err) => error_response(&err),
    }
}
