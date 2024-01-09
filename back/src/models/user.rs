use serde::Deserialize;




#[derive(Deserialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}


impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        let hash = bcrypt::hash(password, 4).unwrap();
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            email,
            password: hash,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
    pub fn verify_password(&self, password: String) -> anyhow::Result<bool> {
        Ok(bcrypt::verify(password, &self.password)?)
    }
}