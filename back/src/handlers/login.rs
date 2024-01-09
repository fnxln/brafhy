use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
pub struct Login {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    user_id: Option<uuid::Uuid>,
    message: String,
    token: Option<String>,
}

pub async fn login_handler(
    State(pool): State<Arc<crate::AppState>>,
    Json(payload): Json<Login>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = match sqlx::query_as!(
        crate::models::user::User,
        "SELECT * FROM users WHERE email = $1",
        payload.email,
    )
    .fetch_one(&pool.db)
    .await
    {
        Ok(user) => user,
        Err(e) => {
            if e.to_string().contains("No rows returned") {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "User does not exist",
                });
                return Err((StatusCode::NOT_FOUND, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "status": "fail",
                    "message": "Something went wrong",
                })),
            ));
        }
    };
    match user.verify_password(payload.password) {
        Ok(true) => {
            let claims = crate::util::jwt::Claims::new(user.id);

            let token = claims.mint().unwrap();

            let login_response = LoginResponse {
                user_id: Some(user.id),
                message: "User logged in successfully".to_string(),
                token: Some(token.clone()),
            };

            let cookie = Cookie::build(("token", token))
                .path("/")
                .same_site(SameSite::Lax)
                .http_only(true);

            let mut response = Response::new(serde_json::to_string(&login_response).unwrap());

            response
                .headers_mut()
                .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
            
            return Ok(response);
        }
        Ok(false) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Invalid password",
            });
            return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
        }
        _ => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "status": "fail",
                    "message": "Something went wrong",
                })),
            ));
        }
    };
}
