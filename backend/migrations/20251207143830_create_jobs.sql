CREATE TABLE jobs (
    id BINARY(16) NOT NULL PRIMARY KEY,
    task_text TEXT NOT NULL,
    state ENUM('Queued', 'Assigned', 'Running', 'Success', 'Failed', 'Retrying') NOT NULL DEFAULT 'Queued',
    agent_id BINARY(16) NULL,
    retries INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (agent_id) REFERENCES agents(id)
);
