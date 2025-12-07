use anyhow::Ok;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::models::job::Job;
use crate::models::job::JobState;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub status: AgentStatus,
    pub last_heartbeat: Option<NaiveDateTime>,
    pub workspace_volume: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "enum('Idle','Busy','Offline')")]
#[sqlx(rename_all = "PascalCase")]
pub enum AgentStatus {
    Idle,
    Busy,
    Offline,
}

#[derive(Debug, Deserialize)]
pub struct CreateAgent {
    pub name: String,
    pub workspace_volume: Option<String>,
}

impl Agent {
    pub async fn get_agent(pool: &MySqlPool, id: Uuid) -> anyhow::Result<Self> {
        
        let agent = sqlx::query_as::<_, Agent>(
            r#"
            SELECT *
            FROM agents 
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_one(pool)
        .await?;
        
        Ok(agent)
    }

    pub async fn create_agent(pool: &MySqlPool, new: CreateAgent) -> anyhow::Result<Self> {
        let id = Uuid::now_v7();
        let status = AgentStatus::Offline;
        
        sqlx::query!(
            r#"
            INSERT INTO agents (id, name, status, workspace_volume)
            VALUES (?,?,?,?)
            "#,
            id,
            new.name,
            status as AgentStatus,
            new.workspace_volume,
        ).execute(pool).await?;

        Ok(Self::get_agent(pool, id).await?)
    }

    pub async fn get_all_agents(pool: &MySqlPool) -> anyhow::Result<Vec<Self>> {

        let agents = sqlx::query_as::<_, Agent>(
            r#"
            SELECT *
            FROM agents 
            ORDER BY id ASC
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(agents)
    }

    pub async fn start_new_job(pool: &MySqlPool, agent: Self) -> anyhow::Result<Job> {
        let job = Job::assign_next_job(pool, agent.id).await?;
        Ok(job)
    }
}
