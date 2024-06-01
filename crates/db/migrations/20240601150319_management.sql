-- migrate:up

CREATE TABLE projects
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR   NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP,
    created_by INT,
    updated_by INT,
    deleted    BOOLEAN   NOT NULL DEFAULT FALSE,
    deleted_at TIMESTAMP,
    deleted_by INT
);

-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON projects
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- migrate:down

