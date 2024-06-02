-- migrate:up

CREATE TABLE users
(
    id                   SERIAL PRIMARY KEY,
    uuid                 UUID      NOT NULL,
    username             VARCHAR   NOT NULL UNIQUE,
    hashed_password      VARCHAR   NOT NULL,
    email                VARCHAR UNIQUE,
    last_organization_id VARCHAR,
    last_project_id      VARCHAR,
    created_at           TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMP
);

-- comments
COMMENT ON COLUMN users.id IS '用户ID';
COMMENT ON COLUMN users.uuid IS '用户唯一标识';
COMMENT ON COLUMN users.username IS '用户名';
COMMENT ON COLUMN users.hashed_password IS '用户密码';
COMMENT ON COLUMN users.email IS '用户邮箱';
COMMENT ON COLUMN users.last_organization_id IS '最后登录的组织ID';
COMMENT ON COLUMN users.last_project_id IS '最后登录的项目ID';
COMMENT ON COLUMN users.created_at IS '创建时间';
COMMENT ON COLUMN users.updated_at IS '更新时间';

-- trigger function: update current_timestamp
CREATE OR REPLACE FUNCTION trigger_set_timestamp()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- migrate:down