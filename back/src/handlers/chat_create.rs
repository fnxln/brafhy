use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Register {
    name: String,
    creator: String,
    password: String,
}
#[derive(Serialize)]
pub struct Response {
    chat_id: Option<uuid::Uuid>,
    message: String,
    token: Option<String>,
}       
pub async fn create_handler(
    State(pool): State<Arc<crate::AppState>>,
    Json(payload): Json<Register>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let new_chat = crate::models::chat::chat::new(payload.name, payload.email, payload.password);
    let query_result = sqlx::query_as!(
        crate::models::chat::chat,
        "INSERT INTO chats (id, name, creator, password, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        new_chat.id,
        new_chat.name,
        new_chat.creator,
        new_chat.password,
        new_chat.created_at,
        new_chat.updated_at,
    )
    .fetch_one(&pool.db)
    .await;
    match query_result {
        Ok(chat) => {
            let claims = crate::util::jwt::Claims::new(chat.id);
            let token = claims.mint().unwrap();
            let response = Response {
                chat_id: Some(chat.id),
                message: "chat created successfully".to_string(),
                token: Some(token),
            };
            return Ok((
                StatusCode::CREATED,
                Json(serde_json::to_value(response).unwrap()),
            ));
        }
        Err(e) => {
            let response = Response {
                chat_id: None,
                message: format!("failed to create chat: {}", e),
                token: None,
            };
            return Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::to_value(response).unwrap()),
            ));
        }
    }
}