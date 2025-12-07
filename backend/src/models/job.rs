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

        let job = Self::get_job(pool, id).await?;

        Ok(job)
    }

    pub async fn get_job(pool: &MySqlPool, id: Uuid) -> anyhow::Result<Self> {
        let job = sqlx::query_as::<_, Job>(
            r#"
            SELECT *
            FROM jobs
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(job)
    }

    pub async fn assign_next_job(pool: &MySqlPool, agent_id: Uuid) -> anyhow::Result<Option<Self>> {
        // This reutrns a job. But there is no guarantee it is the one that just got assigned

        let result = sqlx::query!(
            r#"
            UPDATE jobs 
            SET 
                state = 'Assigned',
                agent_id = ?
            WHERE state = 'Queued'
            AND agent_id IS NULL
            ORDER BY id ASC 
            LIMIT 1
            "#,
            agent_id
        ).execute(pool).await?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }

        let job = sqlx::query_as::<_, Job>(
            r#"
            SELECT *
            FROM jobs 
            WHERE state = 'Assigned'
            AND agent_id = ?
            LIMIT 1
            "#
        )
        .bind(agent_id)
        .fetch_one(pool)
        .await?;

        Ok(Some(job))
    }

    pub async fn set_state(pool: &MySqlPool, id: Uuid, state: JobState) -> anyhow::Result<Self> {
        sqlx::query!(
            r#"
            UPDATE jobs
            SET state = ?
            WHERE id = ?
            "#,
            state as JobState,
            id
        )
        .execute(pool)
        .await?;

        Self::get_job(pool, id).await
    }
}
