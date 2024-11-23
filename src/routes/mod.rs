use crate::controllers::user_controller;
use axum::{routing::post, Router};
use mongodb::Database;

pub fn create_routes(db: Database) -> Router {
    Router::new()
        .route("/signup", post(user_controller::sign_up))
        .route("/login", post(user_controller::login))
        .route("/forgot-password", post(user_controller::forgot_password))
        .with_state(db)
}
