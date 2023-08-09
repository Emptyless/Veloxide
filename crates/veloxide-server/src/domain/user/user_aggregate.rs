use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String, //TODO: consider using email type
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub token_salt: Uuid,
}

impl User {
    pub fn new(email: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            token_salt: Uuid::new_v4(),
        }
    }
}
