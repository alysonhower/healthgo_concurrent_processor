use serde::{Deserialize, Serialize};

// Represents a message stored in the SQLite database
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub worker: i32,
    pub message: String,
    pub interval: i64,
    pub destination_worker: i32,
}
