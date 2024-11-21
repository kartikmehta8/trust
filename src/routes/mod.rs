use crate::controllers::user_controller;
use axum::{routing::get, routing::post, Router};
use mongodb::Database;

pub fn create_routes(db: Database) -> Router {
    Router::new()
        .route("/users", get(user_controller::get_users))
        .route("/users", post(user_controller::add_user))
        .with_state(db)
}
