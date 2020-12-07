use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectResult {
    pub id: Uuid,
    pub text: String
}