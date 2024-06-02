-- migrate:up

CREATE TABLE projects
(
    id             SERIAL PRIMARY KEY,
    name           VARCHAR   NOT NULL,
    created_at     TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMP,
    created_by     INT,
    updated_by     INT,
    deleted        BOOLEAN   NOT NULL DEFAULT FALSE,
    deleted_at     TIMESTAMP,
    deleted_by     INT,
    description    VARCHAR,
    module_setting VARCHAR
);

-- comments

COMMENT ON COLUMN projects.id IS '项目ID';
COMMENT ON COLUMN projects.name IS '项目名称';
COMMENT ON COLUMN projects.created_at IS '创建时间';
COMMENT ON COLUMN projects.updated_at IS '更新时间';
COMMENT ON COLUMN projects.created_by IS '创建人';
COMMENT ON COLUMN projects.updated_by IS '更新人';
COMMENT ON COLUMN projects.deleted IS '是否删除';
COMMENT ON COLUMN projects.deleted_at IS '删除时间';
COMMENT ON COLUMN projects.deleted_by IS '删除人';
COMMENT ON COLUMN projects.description IS '项目描述';
COMMENT ON COLUMN projects.module_setting IS '模块设置';

-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp
    BEFORE UPDATE
    ON projects
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- migrate:down

