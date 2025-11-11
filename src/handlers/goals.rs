use actix_web::{web, HttpResponse, Responder};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Goal {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub target_amount: Decimal,
    pub current_amount: Decimal,
    pub deadline: NaiveDate,
    pub icon: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateGoal {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub target_amount: f64,
    pub deadline: NaiveDate,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateGoal {
    pub name: Option<String>,
    pub target_amount: Option<f64>,
    pub deadline: Option<NaiveDate>,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddProgress {
    pub amount: f64,
}

// GET /api/goals - Listar todas as metas
pub async fn get_all(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
) -> impl Responder {
    let goals = sqlx::query_as::<_, Goal>(
        "SELECT * FROM goals WHERE user_id = ? ORDER BY deadline ASC"
    )
    .bind(user_id.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match goals {
        Ok(goals) => HttpResponse::Ok().json(goals),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch goals"
            }))
        }
    }
}

// GET /api/goals/{id} - Buscar meta por ID
pub async fn get_by_id(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    goal_id: web::Path<String>,
) -> impl Responder {
    let goal = sqlx::query_as::<_, Goal>(
        "SELECT * FROM goals WHERE id = ? AND user_id = ?"
    )
    .bind(goal_id.into_inner())
    .bind(user_id.into_inner())
    .fetch_optional(pool.get_ref())
    .await;

    match goal {
        Ok(Some(goal)) => HttpResponse::Ok().json(goal),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Goal not found"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

// POST /api/goals - Criar nova meta
pub async fn create(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    goal_data: web::Json<CreateGoal>,
) -> impl Responder {
    if let Err(errors) = goal_data.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": errors.to_string()
        }));
    }

    let target = Decimal::from_f64_retain(goal_data.target_amount);
    if target.is_none() || goal_data.target_amount <= 0.0 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid target amount"
        }));
    }

    let icon = goal_data.icon.as_deref().unwrap_or("🎯");
    let goal_id = uuid::Uuid::new_v4().to_string();

    let result = sqlx::query(
        "INSERT INTO goals (id, user_id, name, target_amount, current_amount, deadline, icon)
         VALUES (?, ?, ?, ?, 0, ?, ?)"
    )
    .bind(&goal_id)
    .bind(user_id.into_inner())
    .bind(&goal_data.name)
    .bind(target.unwrap())
    .bind(goal_data.deadline)
    .bind(icon)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let goal = sqlx::query_as::<_, Goal>("SELECT * FROM goals WHERE id = ?")
                .bind(&goal_id)
                .fetch_one(pool.get_ref())
                .await
                .unwrap();
            
            HttpResponse::Created().json(goal)
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create goal"
            }))
        }
    }
}

// PUT /api/goals/{id} - Atualizar meta
pub async fn update(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    goal_id: web::Path<String>,
    update_data: web::Json<UpdateGoal>,
) -> impl Responder {
    let mut query = String::from("UPDATE goals SET ");
    let mut updates = Vec::new();
    let mut has_updates = false;

    if let Some(name) = &update_data.name {
        updates.push(format!("name = '{}'", name));
        has_updates = true;
    }
    if let Some(target) = update_data.target_amount {
        if let Some(decimal) = Decimal::from_f64_retain(target) {
            updates.push(format!("target_amount = {}", decimal));
            has_updates = true;
        }
    }
    if let Some(deadline) = update_data.deadline {
        updates.push(format!("deadline = '{}'", deadline));
        has_updates = true;
    }
    if let Some(icon) = &update_data.icon {
        updates.push(format!("icon = '{}'", icon));
        has_updates = true;
    }

    if !has_updates {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No fields to update"
        }));
    }

    query.push_str(&updates.join(", "));
    query.push_str(&format!(" WHERE id = '{}' AND user_id = '{}'", goal_id.into_inner(), user_id.into_inner()));

    let result = sqlx::query(&query).execute(pool.get_ref()).await;

    match result {
        Ok(result) if result.rows_affected() > 0 => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Goal updated successfully"
            }))
        }
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Goal not found"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update goal"
            }))
        }
    }
}

// POST /api/goals/{id}/progress - Adicionar progresso
pub async fn add_progress(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    goal_id: web::Path<String>,
    progress_data: web::Json<AddProgress>,
) -> impl Responder {
    if progress_data.amount <= 0.0 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Amount must be positive"
        }));
    }

    let amount = Decimal::from_f64_retain(progress_data.amount).unwrap();

    let result = sqlx::query(
        "UPDATE goals 
         SET current_amount = current_amount + ? 
         WHERE id = ? AND user_id = ?"
    )
    .bind(amount)
    .bind(goal_id.into_inner())
    .bind(user_id.into_inner())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) if result.rows_affected() > 0 => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Progress added successfully"
            }))
        }
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Goal not found"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to add progress"
            }))
        }
    }
}

// DELETE /api/goals/{id} - Deletar meta
pub async fn delete(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    goal_id: web::Path<String>,
) -> impl Responder {
    let result = sqlx::query("DELETE FROM goals WHERE id = ? AND user_id = ?")
        .bind(goal_id.into_inner())
        .bind(user_id.into_inner())
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(result) if result.rows_affected() > 0 => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Goal deleted successfully"
            }))
        }
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Goal not found"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete goal"
            }))
        }
    }
}
