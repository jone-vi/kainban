use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Job {
    pub id: Uuid,
    pub task_text: String,
    pub state: JobState,
    pub agent_id: Option<Uuid>,
    pub retries: i32,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "enum('Queued','Assigned','Running','Success','Failed','Retrying')")]
#[sqlx(rename_all = "PascalCase")]
pub enum JobState {
    Queued,
    Assigned,
    Running,
    Success,
    Failed,
    Retrying,
}

#[derive(Debug, Deserialize)]
pub struct CreateJob {
    pub task_text: String,
}

impl Job {
    pub async fn create(pool: &MySqlPool, new: CreateJob) -> anyhow::Result<Self> {
        let id = Uuid::now_v7();
        let state = JobState::Queued;

        sqlx::query!(
            r#"
            INSERT INTO jobs (id, task_text, state, retries)
            VALUES (?, ?, ?, ?)
            "#,
            id,
            new.task_text,
            state as JobState,
            0,
        )
        .execute(pool)
        .await?;

        let job = sqlx::query_as!(
            Job,
            r#"
            SELECT
                id as "id: Uuid",
                task_text,
                state as "state: JobState",
                agent_id as "agent_id?: Uuid",
                retries,
                updated_at as "updated_at: NaiveDateTime"
            FROM jobs
            WHERE id = ?
            "#,
            id,
        )
        .fetch_one(pool)
        .await?;

        Ok(job)
    }
}
