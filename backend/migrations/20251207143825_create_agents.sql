CREATE TABLE agents (
    id BINARY(16) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    status ENUM('Idle', 'Busy', 'Offline') NOT NULL DEFAULT 'Idle',
    last_heartbeat TIMESTAMP NULL,
    workspace_volume VARCHAR(255) NULL
);
