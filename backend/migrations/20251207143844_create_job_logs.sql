CREATE TABLE job_logs (
    id BINARY(16) PRIMARY KEY,
    job_id BINARY(16) NOT NULL,
    message TEXT NOT NULL,
    FOREIGN KEY (job_id) REFERENCES jobs(id)
);
