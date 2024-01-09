use axum::{Extension, response::IntoResponse, http::StatusCode, Json};

use crate::models::user::User;

pub async fn get_me_handler(
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let response = serde_json::json!({
        "status": "success",
        "data": {
            "user": {
                "id": user.id,
                "name": user.name,
                "email": user.email,
                "created_at": user.created_at,
                "updated_at": user.updated_at,
            }
        }
    });
    Ok((StatusCode::OK, Json(response)))
}