-- migrate:up
DROP TABLE IF EXISTS elements;

CREATE TABLE elements
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR   NOT NULL,
    description VARCHAR,
    type        VARCHAR,
    value       VARCHAR,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMP,
    created_by  UUID,
    updated_by  UUID,
    deleted     BOOLEAN   NOT NULL DEFAULT FALSE,
    deleted_at  TIMESTAMP,
    deleted_by  UUID
);
-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp_elements
    BEFORE UPDATE
    ON elements
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- comments

COMMENT ON COLUMN elements.id IS '元素ID';
COMMENT ON COLUMN elements.name IS '元素名称';
COMMENT ON COLUMN elements.description IS '元素描述';
COMMENT ON COLUMN elements.type IS '元素类型';
COMMENT ON COLUMN elements.value IS '元素值';
COMMENT ON COLUMN elements.created_at IS '创建时间';
COMMENT ON COLUMN elements.updated_at IS '更新时间';
COMMENT ON COLUMN elements.created_by IS '创建人';
COMMENT ON COLUMN elements.updated_by IS '更新人';
COMMENT ON COLUMN elements.deleted IS '是否删除';
COMMENT ON COLUMN elements.deleted_at IS '删除时间';
COMMENT ON COLUMN elements.deleted_by IS '删除人';


-- migrate:down

