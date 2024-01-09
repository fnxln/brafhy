use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: NaiveDateTime,
    pub user_id: uuid::Uuid,
    pub exp: usize,
}
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: Option<String>,
}

impl Claims {
    pub fn new(user_id: uuid::Uuid) -> Self {
        Self {
            sub: chrono::Utc::now().naive_utc(),
            user_id,
            exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
        }
    }
    pub fn mint(&self) -> anyhow::Result<String> {
        let jwt_secret = std::env::var("JWT_SECRET")?;
        return Ok(jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &self,
            &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_ref()),
        )?);
    }
}
fn verify_jwt(token: &str) -> anyhow::Result<Claims> {
    let jwt_secret = std::env::var("JWT_SECRET")?;
    let token = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(jwt_secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )?;
    Ok(token.claims)
}

pub async fn auth(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });
    let token = token.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: "fail".to_string(),
            message: Some("You are not logged in, please provide token".to_string()),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;
    info!("token is {}", token);
    let claims = match verify_jwt(&token) {
        Ok(claims) => claims,
        Err(e) => {
            let json_error = ErrorResponse {
                status: "fail".to_string(),
                message: Some(e.to_string()),
            };
            return Err((StatusCode::UNAUTHORIZED, Json(json_error)));
        }
    };
    let user = match sqlx::query_as!(
        crate::models::user::User,
        "SELECT * FROM users WHERE id = $1",
        claims.user_id,
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(user) => user,
        Err(e) => {
            let json_error = ErrorResponse {
                status: "fail".to_string(),
                message: Some(e.to_string()),
            };
            return Err((StatusCode::UNAUTHORIZED, Json(json_error)));
        }
    };
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
