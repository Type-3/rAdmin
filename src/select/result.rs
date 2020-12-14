use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectResult {
    pub id: String,
    pub text: String
}