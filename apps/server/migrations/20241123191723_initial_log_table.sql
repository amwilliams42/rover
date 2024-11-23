-- Add migration script here
CREATE TYPE action_type AS ENUM (
    'Connect', 'Disconnect', 'CreateCall', 
    'UpdateCall', 'DeleteCall', 'OpenCall'
);

CREATE TABLE action_logs (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL,
    action_type action_type NOT NULL,
    user_id TEXT,
    ip_address TEXT,
    details TEXT NOT NULL
);

CREATE INDEX action_logs_timestamp_idx ON action_logs(timestamp);