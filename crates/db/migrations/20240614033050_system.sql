-- migrate:up
DROP TABLE IF EXISTS custom_field;

CREATE TABLE custom_field (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    field_type VARCHAR NOT NULL,
    project_id INT NOT NULL,
    remark VARCHAR,
    internal BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL,
    updated_at TIMESTAMP,
    updated_by UUID,
    deleted BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at TIMESTAMP,
    deleted_by UUID
);

-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp_template
    BEFORE UPDATE
    ON custom_field
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();



DROP TABLE IF EXISTS template;

CREATE TABLE template
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR   NOT NULL,
    project_id  INT       NOT NULL,
    description VARCHAR,
    internal    BOOLEAN   NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by  UUID      NOT NULL,
    updated_at  TIMESTAMP,
    updated_by  UUID,
    deleted     BOOLEAN   NOT NULL DEFAULT FALSE,
    deleted_at  TIMESTAMP,
    deleted_by  UUID
);

-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp_template
    BEFORE UPDATE
    ON template
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- comments

COMMENT ON COLUMN template.id IS '模板ID';
COMMENT ON COLUMN template.name IS '模板名称';
COMMENT ON COLUMN template.description IS '模板描述';
COMMENT ON COLUMN template.internal IS '是否内置模板';
COMMENT ON COLUMN template.created_at IS '创建时间';
COMMENT ON COLUMN template.updated_at IS '更新时间';
COMMENT ON COLUMN template.created_by IS '创建人';
COMMENT ON COLUMN template.updated_by IS '更新人';
COMMENT ON COLUMN template.deleted IS '是否删除';
COMMENT ON COLUMN template.deleted_at IS '删除时间';
COMMENT ON COLUMN template.deleted_by IS '删除人';

DROP TABLE IF EXISTS template_custom_field;

CREATE TABLE template_custom_field
(
    id            SERIAL PRIMARY KEY,
    template_id   INT       NOT NULL,
    name          VARCHAR   NOT NULL,
    required      BOOLEAN   NOT NULL DEFAULT FALSE,
    field_type    VARCHAR   NOT NULL,
    remark        VARCHAR,
    default_value VARCHAR,
    internal      BOOLEAN   NOT NULL DEFAULT FALSE,
    created_at    TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by    UUID      NOT NULL,
    updated_at    TIMESTAMP,
    updated_by    UUID,
    deleted       BOOLEAN   NOT NULL DEFAULT FALSE,
    deleted_at    TIMESTAMP,
    deleted_by    UUID
);

-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp_template_custom_field
    BEFORE UPDATE
    ON template_custom_field
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- comments
COMMENT ON COLUMN template_custom_field.id IS '模版自定义字段ID';
COMMENT ON COLUMN template_custom_field.template_id IS '关联模版ID';
COMMENT ON COLUMN template_custom_field.name IS '模版名称';
COMMENT ON COLUMN template_custom_field.required IS '是否为必要字段';


DROP TABLE IF EXISTS custom_field_option;

CREATE TABLE custom_field_option
(
    id       SERIAL PRIMARY KEY,
    name     VARCHAR NOT NULL,
    field_id INT     NOT NULL,
    value    VARCHAR NOT NULL,
    position INT     NOT NULL
);

-- comments
COMMENT ON COLUMN custom_field_option.id IS '选项值ID';


-- migrate:down
