-- migrate:up

-- table users

DROP TABLE IF EXISTS users;

CREATE TABLE users
(
    id                   SERIAL PRIMARY KEY,
    uuid                 UUID      NOT NULL,
    username             VARCHAR   NOT NULL UNIQUE,
    hashed_password      VARCHAR   NOT NULL,
    email                VARCHAR UNIQUE,
    enable               BOOLEAN DEFAULT FALSE,
    last_project_id      INT,
    created_at           TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMP,
    deleted_at           TIMESTAMP,
    deleted_by           INT
);

-- comments
COMMENT ON COLUMN users.id IS '用户ID';
COMMENT ON COLUMN users.uuid IS '用户唯一标识';
COMMENT ON COLUMN users.username IS '用户名';
COMMENT ON COLUMN users.hashed_password IS '用户密码';
COMMENT ON COLUMN users.email IS '用户邮箱';
COMMENT ON COLUMN users.last_project_id IS '最后登录的项目ID';
COMMENT ON COLUMN users.created_at IS '创建时间';
COMMENT ON COLUMN users.updated_at IS '更新时间';
COMMENT ON COLUMN users.deleted_at IS '删除时间';
COMMENT ON COLUMN users.deleted_by IS '删除执行人';

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
CREATE TRIGGER set_timestamp_user
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- table user_role

DROP TABLE IF EXISTS user_role;

CREATE TABLE user_role
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR   NOT NULL,
    type        VARCHAR   NOT NULL,
    internal    BOOLEAN   NOT NULL DEFAULT FALSE,
    description VARCHAR,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by  UUID,
    updated_at  TIMESTAMP
);

-- comments
COMMENT ON COLUMN user_role.id IS '角色ID';
COMMENT ON COLUMN user_role.name IS '角色名称';
COMMENT ON COLUMN user_role.type IS '所属类型 SYSTEM, ORGANIZATION, PROJECT';
COMMENT ON COLUMN user_role.internal IS '是否内置角色';
COMMENT ON COLUMN user_role.created_at IS '创建时间';
COMMENT ON COLUMN user_role.created_by IS '创建人';
COMMENT ON COLUMN user_role.updated_at IS '更新时间';

-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp_user_role
    BEFORE UPDATE
    ON user_role
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- table user_role_relation

DROP TABLE IF EXISTS user_role_relation;

CREATE TABLE user_role_relation
(
    id              SERIAL PRIMARY KEY,
    user_id         UUID,
    role_id         INT,
    project_id      INT NOT NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by      UUID,
    updated_at      TIMESTAMP,
    updated_by      UUID
);

-- comments
COMMENT ON COLUMN user_role_relation.user_id IS '用户ID';
COMMENT ON COLUMN user_role_relation.role_id IS '角色ID';
COMMENT ON COLUMN user_role_relation.project_id IS '关联项目ID';
COMMENT ON COLUMN user_role_relation.created_at IS '创建时间';
COMMENT ON COLUMN user_role_relation.created_by IS '创建人';
COMMENT ON COLUMN user_role_relation.updated_at IS '更新时间';
COMMENT ON COLUMN user_role_relation.updated_by IS '更新人';

-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp_user_role
    BEFORE UPDATE
    ON user_role_relation
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();


-- table role_permission_relation

DROP TABLE IF EXISTS role_permission_relation;

CREATE TABLE role_permission_relation
(
    id         SERIAL PRIMARY KEY,
    role_id    INT NOT NULL,
    permission_id INT NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);

-- comments
COMMENT ON COLUMN role_permission_relation.id IS '关联关系ID';
COMMENT ON COLUMN role_permission_relation.role_id IS '角色ID';
COMMENT ON COLUMN role_permission_relation.permission_id IS '权限ID';

-- table permission
DROP TABLE IF EXISTS permission

CREATE TABLE permission
(
    id  SERIAL  PRIMARY KEY,
    module  VARCHAR NOT NULL,
    scope   VARCHAR NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);

-- comments
COMMENT ON COLUMN permission.id IS '权限标识ID';
COMMENT ON COLUMN permission.module IS '所属模块';
COMMENT ON COLUMN permission.scope IS '权限控制范围';
COMMENT ON COLUMN permission.created_at IS '创建时间';

-- table api_permission_relation
DROP TABLE IF EXISTS api_permission_relation

CREATE TABLE api_permission_relation
(
    id  SERIAL  PRIMARY KEY,
    uri VARCHAR NOT NULL,
    method  VARCHAR NOT NULL,
    permission_id   INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- comments
COMMENT ON COLUMN api_permission_relation.id IS '关联关系ID';
COMMENT ON COLUMN api_permission_relation.uri IS '关联接口';
COMMENT ON COLUMN api_permission_relation.permission_id IS '权限ID';

-- migrate:down
