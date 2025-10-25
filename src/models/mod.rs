use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

// User models
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub full_name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub cpf: String,
    pub birth_date: NaiveDate,
    pub phone: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 255))]
    pub full_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub cpf: String,
    pub birth_date: NaiveDate,
    pub phone: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

// Transaction models
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: String,
    pub user_id: String,
    pub description: String,
    pub amount: Decimal,
    #[sqlx(rename = "type")]
    pub transaction_type: String,
    pub category_id: Option<String>,
    pub date: chrono::DateTime<chrono::Utc>,
    pub recurring: bool,
    pub recurring_id: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTransaction {
    #[validate(length(min = 1, max = 255))]
    pub description: String,
    pub amount: f64,
    pub transaction_type: String,
    pub category_id: Option<String>,
    pub date: Option<chrono::NaiveDate>,
}

// Category models
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: String,
    pub user_id: Option<String>,
    pub name: String,
    pub icon: String,
    pub color: String,
    #[sqlx(rename = "type")]
    pub category_type: String,
    pub is_default: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategory {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub icon: String,
    pub color: String,
    pub category_type: String,
}

// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
