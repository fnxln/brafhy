use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Chat {
    pub id: uuid::Uuid,
    pub name: String,
    pub creator: uuid::Uuid,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Chat {
    pub fn new(name: String, creator: uuid::Uuid, password: String) -> Self {
        let hash = bcrypt::hash(password, 4).unwrap();
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            creator,
            password: hash,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
    pub fn verify_password(&self, password: String) -> bool {
        bcrypt::verify(password, &self.password).unwrap()
    }
}