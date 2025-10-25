use actix_web::{web, HttpResponse, Responder};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecurringTransaction {
    pub id: String,
    pub user_id: String,
    pub description: String,
    pub amount: Decimal,
    #[sqlx(rename = "type")]
    pub transaction_type: String,
    pub category_id: Option<String>,
    pub frequency: String,
    pub active: bool,
    pub last_generated: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRecurring {
    #[validate(length(min = 1, max = 255))]
    pub description: String,
    pub amount: f64,
    #[validate(custom = "validate_transaction_type")]
    pub transaction_type: String,
    pub category_id: Option<String>,
    #[validate(custom = "validate_frequency")]
    pub frequency: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRecurring {
    pub description: Option<String>,
    pub amount: Option<f64>,
    pub transaction_type: Option<String>,
    pub category_id: Option<String>,
    pub frequency: Option<String>,
    pub active: Option<bool>,
}

fn validate_transaction_type(value: &str) -> Result<(), validator::ValidationError> {
    if value == "income" || value == "expense" {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_type"))
    }
}

fn validate_frequency(value: &str) -> Result<(), validator::ValidationError> {
    if ["daily", "weekly", "monthly", "yearly"].contains(&value) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_frequency"))
    }
}

// GET /api/recurring - Listar todas
pub async fn get_all(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
) -> impl Responder {
    let recurring = sqlx::query_as::<_, RecurringTransaction>(
        "SELECT * FROM recurring_transactions WHERE user_id = ? ORDER BY created_at DESC"
    )
    .bind(user_id.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match recurring {
        Ok(recurring) => HttpResponse::Ok().json(recurring),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch recurring transactions"
            }))
        }
    }
}

// POST /api/recurring - Criar nova
pub async fn create(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    recurring_data: web::Json<CreateRecurring>,
) -> impl Responder {
    if let Err(errors) = recurring_data.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": errors.to_string()
        }));
    }

    let amount = Decimal::from_f64_retain(recurring_data.amount);
    if amount.is_none() || recurring_data.amount <= 0.0 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid amount"
        }));
    }

    let recurring_id = uuid::Uuid::new_v4().to_string();

    let result = sqlx::query(
        "INSERT INTO recurring_transactions (id, user_id, description, amount, type, category_id, frequency, active)
         VALUES (?, ?, ?, ?, ?, ?, ?, TRUE)"
    )
    .bind(&recurring_id)
    .bind(user_id.into_inner())
    .bind(&recurring_data.description)
    .bind(amount.unwrap())
    .bind(&recurring_data.transaction_type)
    .bind(&recurring_data.category_id)
    .bind(&recurring_data.frequency)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let recurring = sqlx::query_as::<_, RecurringTransaction>(
                "SELECT * FROM recurring_transactions WHERE id = ?"
            )
            .bind(&recurring_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap();
            
            HttpResponse::Created().json(recurring)
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create recurring transaction"
            }))
        }
    }
}

// PUT /api/recurring/{id} - Atualizar
pub async fn update(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    recurring_id: web::Path<String>,
    update_data: web::Json<UpdateRecurring>,
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
    if let Some(freq) = &update_data.frequency {
        updates.push(format!("frequency = '{}'", freq));
    }
    if let Some(active) = update_data.active {
        updates.push(format!("active = {}", active));
    }

    if updates.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No fields to update"
        }));
    }

    let query = format!(
        "UPDATE recurring_transactions SET {} WHERE id = '{}' AND user_id = '{}'",
        updates.join(", "),
        recurring_id.into_inner(),
        user_id.into_inner()
    );

    let result = sqlx::query(&query).execute(pool.get_ref()).await;

    match result {
        Ok(result) if result.rows_affected() > 0 => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Recurring transaction updated successfully"
            }))
        }
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Recurring transaction not found"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update recurring transaction"
            }))
        }
    }
}

// DELETE /api/recurring/{id} - Deletar
pub async fn delete(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    recurring_id: web::Path<String>,
) -> impl Responder {
    let result = sqlx::query(
        "DELETE FROM recurring_transactions WHERE id = ? AND user_id = ?"
    )
    .bind(recurring_id.into_inner())
    .bind(user_id.into_inner())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) if result.rows_affected() > 0 => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Recurring transaction deleted successfully"
            }))
        }
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Recurring transaction not found"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete recurring transaction"
            }))
        }
    }
}

// POST /api/recurring/generate - Gerar transações pendentes
pub async fn generate_pending(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
) -> impl Responder {
    // Buscar recorrências ativas
    let recurring_list = sqlx::query_as::<_, RecurringTransaction>(
        "SELECT * FROM recurring_transactions WHERE user_id = ? AND active = TRUE"
    )
    .bind(user_id.as_ref() as &str)
    .fetch_all(pool.get_ref())
    .await;

    let recurring_list = match recurring_list {
        Ok(list) => list,
        Err(e) => {
            eprintln!("Database error: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch recurring transactions"
            }));
        }
    };

    let mut generated_count = 0;
    let now = chrono::Utc::now();

    for recurring in recurring_list {
        // Verificar se precisa gerar
        let should_generate = match recurring.last_generated {
            None => true,
            Some(last) => {
                let days_since = (now - last).num_days();
                match recurring.frequency.as_str() {
                    "daily" => days_since >= 1,
                    "weekly" => days_since >= 7,
                    "monthly" => days_since >= 30,
                    "yearly" => days_since >= 365,
                    _ => false,
                }
            }
        };

        if should_generate {
            // Criar transação
            let transaction_id = uuid::Uuid::new_v4().to_string();
            let result = sqlx::query(
                "INSERT INTO transactions (id, user_id, description, amount, type, category_id, date, recurring, recurring_id)
                 VALUES (?, ?, ?, ?, ?, ?, ?, TRUE, ?)"
            )
            .bind(&transaction_id)
            .bind(&recurring.user_id)
            .bind(&recurring.description)
            .bind(&recurring.amount)
            .bind(&recurring.transaction_type)
            .bind(&recurring.category_id)
            .bind(now)
            .bind(&recurring.id)
            .execute(pool.get_ref())
            .await;

            if result.is_ok() {
                // Atualizar last_generated
                let _ = sqlx::query(
                    "UPDATE recurring_transactions SET last_generated = ? WHERE id = ?"
                )
                .bind(now)
                .bind(&recurring.id)
                .execute(pool.get_ref())
                .await;

                generated_count += 1;
            }
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "message": format!("{} transactions generated", generated_count),
        "count": generated_count
    }))
}
