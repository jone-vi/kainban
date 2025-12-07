use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct JobLog {
    pub id: Uuid,
    pub message: String,
    pub job_id: Uuid,
}
