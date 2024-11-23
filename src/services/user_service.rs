use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use mongodb::{bson::doc, Database};
use rand::Rng;
use serde::Serialize;
use std::env;

use crate::models::user::User;

#[derive(Debug, Serialize)]
struct Claims {
    email: String,
    exp: usize,
}

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Failed to hash password")
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    verify(password, hashed_password).unwrap_or(false)
}

pub fn generate_jwt(email: &str) -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        email: email.to_string(),
        exp: expiration,
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub async fn sign_up(db: &Database, email: String, password: String) -> Result<(), String> {
    let collection = db.collection::<User>("users");

    let user_exists = collection
        .find_one(doc! { "email": &email })
        .await
        .map_err(|_| "Database query failed")?
        .is_some();

    if user_exists {
        return Err("Email already exists".to_string());
    }

    let hashed_password = hash_password(&password);
    let user = User {
        id: Some(mongodb::bson::oid::ObjectId::new()),
        email,
        password: hashed_password,
        reset_code: None,
    };

    collection
        .insert_one(user)
        .await
        .map_err(|_| "Failed to insert user into database")?;

    Ok(())
}

pub async fn login(db: &Database, email: String, password: String) -> Result<String, String> {
    let collection = db.collection::<User>("users");

    let user = collection
        .find_one(doc! { "email": &email })
        .await
        .map_err(|_| "Database query failed")?
        .ok_or_else(|| "Invalid email or password".to_string())?;

    if !verify_password(&password, &user.password) {
        return Err("Invalid email or password".to_string());
    }

    let token = generate_jwt(&email);
    Ok(token)
}

pub async fn forgot_password(db: &Database, email: String) -> Result<(), String> {
    let collection = db.collection::<User>("users");

    let user_exists = collection
        .find_one(doc! { "email": &email })
        .await
        .map_err(|_| "Database query failed")?
        .is_some();

    if !user_exists {
        return Err("Email does not exist".to_string());
    }

    let reset_code: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    collection
        .update_one(
            doc! { "email": &email },
            doc! { "$set": { "reset_code": &reset_code } },
        )
        .await
        .map_err(|_| "Failed to update reset code")?;

    let is_email_sent = send_email(&email, &reset_code).await;

    if is_email_sent.is_err() {
        println!("{}", is_email_sent.err().unwrap());
        return Err("Failed to send email".to_string());
    } else {
        return Ok(());
    }
}

async fn send_email(email: &str, reset_code: &str) -> Result<(), String> {
    use lettre::message::Mailbox;
    use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

    let smtp_email = env::var("SMTP_EMAIL").expect("SMTP_EMAIL must be set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

    let email_body = format!(
        "Your password reset code is: {}\nThis code is valid for 15 minutes.",
        reset_code
    );

    let email = Message::builder()
        .from(Mailbox::new(None, smtp_email.parse().unwrap()))
        .to(Mailbox::new(None, email.parse().unwrap()))
        .subject("Password Reset Code")
        .body(email_body)
        .map_err(|_| "Failed to build email message")?;

    let creds = Credentials::new(smtp_email, smtp_password);

    let mailer = SmtpTransport::builder_dangerous("smtp.gmail.com")
        .port(587)
        .credentials(creds)
        .build();

    mailer.send(&email).map_err(|_| "Failed to send email")?;

    Ok(())
}
