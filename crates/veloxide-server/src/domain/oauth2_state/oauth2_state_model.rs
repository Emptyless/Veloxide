use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct OAuth2State {
    pub id: Uuid,
    pub csrf_state: String,
    pub code_verifier: String,
    pub return_url: String,
    pub created_at: DateTime<Utc>,
}

impl OAuth2State {
    pub fn new(csrf_state: String, code_verifier: String, return_url: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            csrf_state,
            code_verifier,
            return_url,
            created_at: Utc::now(),
        }
    }
}
