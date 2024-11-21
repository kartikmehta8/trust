use mongodb::{options::ClientOptions, Client, Database};
use std::env;

pub async fn init() -> Result<Database, mongodb::error::Error> {
    let uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    let client_options = ClientOptions::parse(&uri).await?;
    let client = Client::with_options(client_options)?;
    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    Ok(client.database(&db_name))
}
