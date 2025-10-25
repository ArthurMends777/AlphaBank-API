use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::MySqlPool;
use validator::Validate;

use crate::models::{LoginRequest, RegisterRequest, User};
use crate::utils::{create_jwt, hash_password, validate_cpf, verify_password};

#[derive(Debug, Deserialize)]
pub struct UpdateProfile {
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub birth_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePassword {
    #[validate(length(min = 6))]
    pub old_password: String,
    #[validate(length(min = 6))]
    pub new_password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ForgotPasswordRequest {
    #[validate(email)]
    pub email: String,
}

pub async fn register(
    pool: web::Data<MySqlPool>,
    user_data: web::Json<RegisterRequest>,
) -> impl Responder {
    if let Err(errors) = user_data.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": errors.to_string()
        }));
    }

    if !validate_cpf(&user_data.cpf) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid CPF"
        }));
    }

    let email_exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE email = ?")
        .bind(&user_data.email)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    if email_exists > 0 {
        return HttpResponse::Conflict().json(serde_json::json!({
            "error": "Email already registered"
        }));
    }

    let cpf_exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE cpf = ?")
        .bind(&user_data.cpf)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

    if cpf_exists > 0 {
        return HttpResponse::Conflict().json(serde_json::json!({
            "error": "CPF already registered"
        }));
    }

    let password_hash = match hash_password(&user_data.password) {
        Ok(hash) => hash,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to hash password"
            }));
        }
    };

    let user_id = uuid::Uuid::new_v4().to_string();

    let result = sqlx::query(
        "INSERT INTO users (id, full_name, email, password_hash, cpf, birth_date, phone)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&user_id)
    .bind(&user_data.full_name)
    .bind(&user_data.email)
    .bind(&password_hash)
    .bind(&user_data.cpf)
    .bind(user_data.birth_date)
    .bind(&user_data.phone)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
                .bind(&user_id)
                .fetch_one(pool.get_ref())
                .await
                .unwrap();

            let token = create_jwt(&user.id).unwrap();

            HttpResponse::Created().json(serde_json::json!({
                "token": token,
                "user": user
            }))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create user"
            }))
        }
    }
}

pub async fn login(
    pool: web::Data<MySqlPool>,
    credentials: web::Json<LoginRequest>,
) -> impl Responder {
    if let Err(errors) = credentials.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": errors.to_string()
        }));
    }

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&credentials.email)
        .fetch_optional(pool.get_ref())
        .await;

    let user = match user {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid credentials"
            }));
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }));
        }
    };

    match verify_password(&credentials.password, &user.password_hash) {
        Ok(true) => {
            let token = create_jwt(&user.id).unwrap();
            HttpResponse::Ok().json(serde_json::json!({
                "token": token,
                "user": user
            }))
        }
        Ok(false) => {
            HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid credentials"
            }))
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Authentication error"
            }))
        }
    }
}

pub async fn me(pool: web::Data<MySqlPool>, user_id: web::ReqData<String>) -> impl Responder {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(user_id.into_inner())
        .fetch_optional(pool.get_ref())
        .await;

    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

pub async fn update_profile(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    update_data: web::Json<UpdateProfile>,
) -> impl Responder {
    let mut updates = Vec::new();

    if let Some(name) = &update_data.full_name {
        updates.push(format!("full_name = '{}'", name));
    }
    if let Some(email) = &update_data.email {
        let uid: &str = user_id.as_ref();
        let exists = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM users WHERE email = ? AND id != ?"
        )
        .bind(email)
        .bind(uid)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(0);

        if exists > 0 {
            return HttpResponse::Conflict().json(serde_json::json!({
                "error": "Email already in use"
            }));
        }
        updates.push(format!("email = '{}'", email));
    }
    if let Some(phone) = &update_data.phone {
        updates.push(format!("phone = '{}'", phone));
    }
    if let Some(birth_date) = update_data.birth_date {
        updates.push(format!("birth_date = '{}'", birth_date));
    }

    if updates.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No fields to update"
        }));
    }

    let uid = user_id.into_inner();
    let query = format!(
        "UPDATE users SET {} WHERE id = '{}'",
        updates.join(", "),
        uid
    );

    let result = sqlx::query(&query).execute(pool.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Profile updated successfully"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update profile"
            }))
        }
    }
}

pub async fn change_password(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    password_data: web::Json<ChangePassword>,
) -> impl Responder {
    if let Err(errors) = password_data.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": errors.to_string()
        }));
    }

    let uid: &str = user_id.as_ref();
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(uid)
        .fetch_optional(pool.get_ref())
        .await;

    let user = match user {
        Ok(Some(user)) => user,
        _ => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "error": "User not found"
            }));
        }
    };

    match verify_password(&password_data.old_password, &user.password_hash) {
        Ok(true) => {},
        Ok(false) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Current password is incorrect"
            }));
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Authentication error"
            }));
        }
    }

    let new_hash = match hash_password(&password_data.new_password) {
        Ok(hash) => hash,
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to hash password"
            }));
        }
    };

    let result = sqlx::query("UPDATE users SET password_hash = ? WHERE id = ?")
        .bind(&new_hash)
        .bind(user_id.into_inner())
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Password changed successfully"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to change password"
            }))
        }
    }
}

pub async fn forgot_password(
    pool: web::Data<MySqlPool>,
    request_data: web::Json<ForgotPasswordRequest>,
) -> impl Responder {
    if let Err(errors) = request_data.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": errors.to_string()
        }));
    }

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&request_data.email)
        .fetch_optional(pool.get_ref())
        .await;

    match user {
        Ok(Some(_user)) => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": "If the email exists, a recovery link will be sent",
                "note": "Email sending not implemented yet"
            }))
        }
        Ok(None) => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": "If the email exists, a recovery link will be sent"
            }))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}
