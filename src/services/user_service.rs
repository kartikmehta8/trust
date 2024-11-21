use crate::models::user::User;
use futures_util::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::Document, error::Result, Database};

pub async fn get_all_users(db: &Database) -> Result<Vec<User>> {
    let collection = db.collection::<User>("users");
    let cursor = collection.find(Document::default()).await?;
    let users: Vec<User> = cursor.try_collect().await?;
    Ok(users)
}

pub async fn add_user(db: &Database, name: String, email: String) -> Result<()> {
    let collection = db.collection::<User>("users");
    let user = User {
        id: Some(ObjectId::new()),
        name,
        email,
    };
    collection.insert_one(user).await?;
    Ok(())
}
