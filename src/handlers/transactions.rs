use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::models::{CreateTransaction, Transaction};

#[derive(Debug, Deserialize)]
pub struct UpdateTransaction {
    pub description: Option<String>,
    pub amount: Option<f64>,
    pub transaction_type: Option<String>,
    pub category_id: Option<String>,
}

// GET /api/transactions - Listar todas
pub async fn get_all(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
) -> impl Responder {
    let transactions = sqlx::query_as::<_, Transaction>(
        "SELECT * FROM transactions WHERE user_id = ? ORDER BY date DESC"
    )
    .bind(user_id.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match transactions {
        Ok(transactions) => HttpResponse::Ok().json(transactions),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch transactions"
            }))
        }
    }
}

// GET /api/transactions/{id} - Buscar por ID
pub async fn get_by_id(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    transaction_id: web::Path<String>,
) -> impl Responder {
    let transaction = sqlx::query_as::<_, Transaction>(
        "SELECT * FROM transactions WHERE id = ? AND user_id = ?"
    )
    .bind(transaction_id.into_inner())
    .bind(user_id.into_inner())
    .fetch_optional(pool.get_ref())
    .await;

    match transaction {
        Ok(Some(transaction)) => HttpResponse::Ok().json(transaction),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Transaction not found"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

// POST /api/transactions - Criar nova
pub async fn create(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    transaction_data: web::Json<CreateTransaction>,
) -> impl Responder {
    let amount = Decimal::from_f64_retain(transaction_data.amount);
    if amount.is_none() || transaction_data.amount == 0.0 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid amount"
        }));
    }

    let transaction_id = uuid::Uuid::new_v4().to_string();
    let date = transaction_data.date
        .map(|d| d.and_hms_opt(0, 0, 0).unwrap())
        .unwrap_or_else(|| Utc::now().naive_utc());

    let result = sqlx::query(
        "INSERT INTO transactions (id, user_id, description, amount, type, category_id, date, recurring)
         VALUES (?, ?, ?, ?, ?, ?, ?, FALSE)"
    )
    .bind(&transaction_id)
    .bind(user_id.into_inner())
    .bind(&transaction_data.description)
    .bind(amount.unwrap())
    .bind(&transaction_data.transaction_type)
    .bind(&transaction_data.category_id)
    .bind(date)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let transaction = sqlx::query_as::<_, Transaction>(
                "SELECT * FROM transactions WHERE id = ?"
            )
            .bind(&transaction_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap();
            
            HttpResponse::Created().json(transaction)
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create transaction"
            }))
        }
    }
}

// PUT /api/transactions/{id} - Atualizar
pub async fn update(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    transaction_id: web::Path<String>,
    update_data: web::Json<UpdateTransaction>,
) -> impl Responder {
    let mut updates = Vec::new();

    if let Some(desc) = &update_data.description {
        updates.push(format!("description = '{}'", desc));
    }
    if let Some(amount) = update_data.amount {
        if let Some(decimal) = Decimal::from_f64_retain(amount) {
            updates.push(format!("amount = {}", decimal));
        }
    }
    if let Some(t_type) = &update_data.transaction_type {
        updates.push(format!("type = '{}'", t_type));
    }
    if let Some(cat_id) = &update_data.category_id {
        updates.push(format!("category_id = '{}'", cat_id));
    }

    if updates.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No fields to update"
        }));
    }

    let query = format!(
        "UPDATE transactions SET {} WHERE id = '{}' AND user_id = '{}'",
        updates.join(", "),
        transaction_id.into_inner(),
        user_id.into_inner()
    );

    let result = sqlx::query(&query).execute(pool.get_ref()).await;

    match result {
        Ok(result) if result.rows_affected() > 0 => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Transaction updated successfully"
            }))
        }
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Transaction not found"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update transaction"
            }))
        }
    }
}

// DELETE /api/transactions/{id} - Deletar
pub async fn delete(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    transaction_id: web::Path<String>,
) -> impl Responder {
    let result = sqlx::query(
        "DELETE FROM transactions WHERE id = ? AND user_id = ?"
    )
    .bind(transaction_id.into_inner())
    .bind(user_id.into_inner())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) if result.rows_affected() > 0 => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Transaction deleted successfully"
            }))
        }
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Transaction not found"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete transaction"
            }))
        }
    }
}
