ALTER TABLE agents
    ADD COLUMN api_token_hash BINARY(32) NOT NULL AFTER workspace_volume,
    ADD INDEX idx_agents_api_token_hash (api_token_hash);

CREATE TABLE users (
    id BINARY(16) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    api_token_hash BINARY(32) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_users_api_token_hash (api_token_hash)
);
