use actix_web::{web, HttpResponse, Responder};
use sqlx::MySqlPool;
use validator::Validate;

use crate::models::{Category, CreateCategory};

pub async fn get_all(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
) -> impl Responder {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories 
         WHERE is_default = TRUE OR user_id = ? 
         ORDER BY is_default DESC, name ASC"
    )
    .bind(user_id.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match categories {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch categories"
            }))
        }
    }
}

pub async fn create(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    category_data: web::Json<CreateCategory>,
) -> impl Responder {
    if let Err(errors) = category_data.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Validation failed",
            "details": errors.to_string()
        }));
    }

    let icon = if category_data.icon.is_empty() {
        "💵"
    } else {
        &category_data.icon
    };

    let color = if category_data.color.is_empty() {
        "#636e72"
    } else {
        &category_data.color
    };
    let category_id = uuid::Uuid::new_v4().to_string();

    let result = sqlx::query(
        "INSERT INTO categories (id, user_id, name, icon, color, type)
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&category_id)
    .bind(user_id.into_inner())
    .bind(&category_data.name)
    .bind(icon)
    .bind(color)
    .bind(&category_data.category_type)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            let category = sqlx::query_as::<_, Category>(
                "SELECT * FROM categories WHERE id = ?"
            )
            .bind(&category_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap();
            
            HttpResponse::Created().json(category)
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create category"
            }))
        }
    }
}

pub async fn delete(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    category_id: web::Path<String>,
) -> impl Responder {
    let result = sqlx::query(
        "DELETE FROM categories 
         WHERE id = ? AND user_id = ? AND is_default = FALSE"
    )
    .bind(category_id.into_inner())
    .bind(user_id.into_inner())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) if result.rows_affected() > 0 => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Category deleted successfully"
            }))
        }
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Category not found"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete category"
            }))
        }
    }
}

// PUT /api/categories/{id} - Atualizar categoria
pub async fn update(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    category_id: web::Path<String>,
    update_data: web::Json<CreateCategory>,
) -> impl Responder {
    let mut updates = Vec::new();

    if !update_data.name.is_empty() {
        updates.push(format!("name = '{}'", update_data.name));
    }
    if !update_data.icon.is_empty() {
        updates.push(format!("icon = '{}'", update_data.icon));
    }
    if !update_data.color.is_empty() {
        updates.push(format!("color = '{}'", update_data.color));
    }

    if updates.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No fields to update"
        }));
    }

    let query = format!(
        "UPDATE categories SET {} WHERE id = '{}' AND user_id = '{}'",
        updates.join(", "),
        category_id.into_inner(),
        user_id.into_inner()
    );

    let result = sqlx::query(&query).execute(pool.get_ref()).await;

    match result {
        Ok(result) if result.rows_affected() > 0 => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Category updated successfully"
            }))
        }
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Category not found or not owned by user"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update category"
            }))
        }
    }
}
