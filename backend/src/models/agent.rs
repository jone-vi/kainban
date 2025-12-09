use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::models::job::Job;
use crate::token;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub status: AgentStatus,
    pub last_heartbeat: Option<NaiveDateTime>,
    pub workspace_volume: Option<String>,
    #[serde(skip_serializing)]
    pub api_token_hash: Vec<u8>,
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

#[derive(Debug, Serialize)]
pub struct AgentWithToken {
    pub agent: Agent,
    pub token: String,
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

    pub async fn create_agent(pool: &MySqlPool, new: CreateAgent) -> anyhow::Result<AgentWithToken> {
        let id = Uuid::now_v7();
        let status = AgentStatus::Offline;
        let token = token::generate_token();
        let token_hash = token::hash_token(&token);
        sqlx::query(
            r#"
            INSERT INTO agents (id, name, status, workspace_volume, api_token_hash)
            VALUES (?,?,?,?,?)
            "#,
        )
        .bind(id)
        .bind(new.name)
        .bind(status)
        .bind(new.workspace_volume)
        .bind(token_hash.as_slice())
        .execute(pool)
        .await?;

        let agent = Self::get_agent(pool, id).await?;

        Ok(AgentWithToken { agent, token })
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

    pub async fn start_new_job(pool: &MySqlPool, agent: Self) -> anyhow::Result<Option<Job>> {
        let job = Job::assign_next_job(pool, agent.id).await?;
        Ok(job)
    }

    pub async fn find_by_token(pool: &MySqlPool, token: &str) -> anyhow::Result<Option<Self>> {
        let token_hash = token::hash_token(token);

        let agent = sqlx::query_as::<_, Agent>(
            r#"
            SELECT *
            FROM agents 
            WHERE api_token_hash = ?
            LIMIT 1
            "#
        )
        .bind(token_hash.as_slice())
        .fetch_optional(pool)
        .await?;

        Ok(agent)
    }
}
