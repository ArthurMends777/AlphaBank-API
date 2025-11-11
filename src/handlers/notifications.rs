use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub message: String,
    #[sqlx(rename = "type")]
    pub notification_type: String,
    pub read: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNotification {
    pub title: String,
    pub message: String,
    pub notification_type: Option<String>,
}

// GET /api/notifications - Listar todas
pub async fn get_all(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
) -> impl Responder {
    let notifications = sqlx::query_as::<_, Notification>(
        "SELECT * FROM notifications WHERE user_id = ? ORDER BY created_at DESC"
    )
    .bind(user_id.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match notifications {
        Ok(notifications) => HttpResponse::Ok().json(notifications),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch notifications"
            }))
        }
    }
}

// POST /api/notifications - Criar notificação
pub async fn create(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    notif_data: web::Json<CreateNotification>,
) -> impl Responder {
    let notif_id = uuid::Uuid::new_v4().to_string();
    let notif_type = notif_data.notification_type.as_deref().unwrap_or("info");

    let result = sqlx::query(
        "INSERT INTO notifications (id, user_id, title, message, type, `read`)
         VALUES (?, ?, ?, ?, ?, FALSE)"
    )
    .bind(&notif_id)
    .bind(user_id.into_inner())
    .bind(&notif_data.title)
    .bind(&notif_data.message)
    .bind(notif_type)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "id": notif_id,
            "message": "Notification created"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create notification"
            }))
        }
    }
}

// PUT /api/notifications/{id}/read - Marcar como lida
pub async fn mark_as_read(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    notif_id: web::Path<String>,
) -> impl Responder {
    let result = sqlx::query(
        "UPDATE notifications SET `read` = TRUE WHERE id = ? AND user_id = ?"
    )
    .bind(notif_id.into_inner())
    .bind(user_id.into_inner())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) if result.rows_affected() > 0 => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Notification marked as read"
            }))
        }
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Notification not found"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update notification"
            }))
        }
    }
}

// DELETE /api/notifications/{id} - Deletar
pub async fn delete(
    pool: web::Data<MySqlPool>,
    user_id: web::ReqData<String>,
    notif_id: web::Path<String>,
) -> impl Responder {
    let result = sqlx::query(
        "DELETE FROM notifications WHERE id = ? AND user_id = ?"
    )
    .bind(notif_id.into_inner())
    .bind(user_id.into_inner())
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(result) if result.rows_affected() > 0 => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Notification deleted"
            }))
        }
        Ok(_) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Notification not found"
        })),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete notification"
            }))
        }
    }
}
