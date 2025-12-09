use axum::{
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap, StatusCode},
};
use sqlx::MySqlPool;

use crate::models::{agent::Agent, user::User};

#[derive(Debug)]
pub struct AuthenticatedAgent(pub Agent);

#[derive(Debug)]
pub struct AuthenticatedUser(pub User);

#[derive(Debug)]
pub enum AuthError {
    Missing,
    Invalid,
    Internal,
}

impl From<AuthError> for (StatusCode, &'static str) {
    fn from(value: AuthError) -> Self {
        value.status_and_msg()
    }
}

impl AuthError {
    fn status_and_msg(&self) -> (StatusCode, &'static str) {
        match self {
            AuthError::Missing => (StatusCode::UNAUTHORIZED, "Missing bearer token"),
            AuthError::Invalid => (StatusCode::UNAUTHORIZED, "Invalid bearer token"),
            AuthError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Authentication failed"),
        }
    }
}

impl FromRequestParts<MySqlPool> for AuthenticatedAgent {
    type Rejection = (StatusCode, &'static str);

    fn from_request_parts(
        parts: &mut Parts,
        pool: &MySqlPool,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        let headers = parts.headers.clone();
        let pool = pool.clone();
        async move {
            let token = extract_bearer(&headers).ok_or(AuthError::Missing)?;

            let agent = Agent::find_by_token(&pool, &token)
                .await
                .map_err(|_| AuthError::Internal)?
                .ok_or(AuthError::Invalid)?;

            Ok(Self(agent))
        }
    }
}

impl FromRequestParts<MySqlPool> for AuthenticatedUser {
    type Rejection = (StatusCode, &'static str);

    fn from_request_parts(
        parts: &mut Parts,
        pool: &MySqlPool,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        let headers = parts.headers.clone();
        let pool = pool.clone();
        async move {
            let token = extract_bearer(&headers).ok_or(AuthError::Missing)?;

            let user = User::find_by_token(&pool, &token)
                .await
                .map_err(|_| AuthError::Internal)?
                .ok_or(AuthError::Invalid)?;

            Ok(Self(user))
        }
    }
}

fn extract_bearer(headers: &HeaderMap) -> Option<String> {
    let value = headers
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?;

    value
        .strip_prefix("Bearer ")
        .map(|token| token.trim().to_string())
}
