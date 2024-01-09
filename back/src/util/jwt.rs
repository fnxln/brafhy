use chrono::{Duration, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: NaiveDateTime,
    pub user_id: uuid::Uuid,
    pub exp: NaiveDateTime,
}

impl Claims {
    pub fn new(user_id: uuid::Uuid) -> Self {
        Self {
            sub: chrono::Utc::now().naive_utc(),
            user_id,
            exp: (chrono::Utc::now() + chrono::Duration::days(1)).naive_utc(),
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
