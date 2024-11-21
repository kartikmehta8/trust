use dotenv::dotenv;

mod config;
mod controllers;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = config::database::init().await.unwrap();

    let app = routes::create_routes(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on port 3000");
    axum::serve(listener, app).await.unwrap();
}
