use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Register {
    name: String,
    email: String,
    password: String,
}
#[derive(Serialize)]
pub struct Response {
    user_id: Option<uuid::Uuid>,
    message: String,
    token: Option<String>,
}
pub async fn register_handler(
    State(pool): State<Arc<crate::AppState>>,
    Json(payload): Json<Register>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let new_user = crate::models::user::User::new(payload.name, payload.email, payload.password);
    let query_result = sqlx::query_as!(
        crate::models::user::User,
        "INSERT INTO users (id, name, email, password, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        new_user.id,
        new_user.name,
        new_user.email,
        new_user.password,
        new_user.created_at,
        new_user.updated_at,
    )
    .fetch_one(&pool.db)
    .await;
    match query_result {
        Ok(user) => {
            let claims = crate::util::jwt::Claims::new(user.id);
            let token = claims.mint().unwrap();
            let response = Response {
                user_id: Some(user.id),
                message: "User created successfully".to_string(),
                token: Some(token),
            };
            return Ok((StatusCode::CREATED, Json(serde_json::to_value(response).unwrap())));
        },
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "User already exitsts",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "status": "fail",
                    "message": "Something went wrong",
                })),
            ));
        }
    }
}
