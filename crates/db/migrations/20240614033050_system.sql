-- migrate:up
DROP TABLE IF EXISTS field;

CREATE TABLE field (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    label VARCHAR,
    field_type VARCHAR NOT NULL,
    project_id INT NOT NULL,
    remark VARCHAR,
    internal BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    created_by UUID NOT NULL,
    updated_at TIMESTAMP,
    updated_by UUID,
    deleted_at TIMESTAMP,
    deleted_by UUID
);

-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp_field BEFORE
UPDATE ON field FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp ();

DROP TABLE IF EXISTS template;

CREATE TABLE template (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    project_id INT NOT NULL,
    description VARCHAR,
    internal BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    created_by UUID NOT NULL,
    updated_at TIMESTAMP,
    updated_by UUID,
    deleted_at TIMESTAMP,
    deleted_by UUID
);

-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp_template BEFORE
UPDATE ON template FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp ();

-- comments
COMMENT ON COLUMN template.id IS '模板ID';

COMMENT ON COLUMN template.name IS '模板名称';

COMMENT ON COLUMN template.description IS '模板描述';

COMMENT ON COLUMN template.internal IS '是否内置模板';

COMMENT ON COLUMN template.created_at IS '创建时间';

COMMENT ON COLUMN template.updated_at IS '更新时间';

COMMENT ON COLUMN template.created_by IS '创建人';

COMMENT ON COLUMN template.updated_by IS '更新人';

COMMENT ON COLUMN template.deleted_at IS '删除时间';

COMMENT ON COLUMN template.deleted_by IS '删除人';

DROP TABLE IF EXISTS template_field_relation;

CREATE TABLE template_field_relation (
    id SERIAL PRIMARY KEY,
    template_id INT NOT NULL,
    field_id INT NOT NULL,
    default_value VARCHAR,
    required BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    created_by UUID NOT NULL,
    updated_at TIMESTAMP,
    updated_by UUID,
    deleted_at TIMESTAMP,
    deleted_by UUID
);

-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp_template_field_relation BEFORE
UPDATE ON template_field_relation FOR EACH ROW EXECUTE PROCEDURE trigger_set_timestamp ();

-- comments
COMMENT ON COLUMN template_field_relation.id IS '模版字段关联关系ID';

COMMENT ON COLUMN template_field_relation.template_id IS '关联模版ID';

COMMENT ON COLUMN template_field_relation.field_id IS '关联字段ID';

COMMENT ON COLUMN template_field_relation.required IS '是否为必填字段';

COMMENT ON COLUMN template_field_relation.default_value IS '默认值';

COMMENT ON COLUMN template_field_relation.created_at IS '创建时间';

COMMENT ON COLUMN template_field_relation.created_by IS '创建人';

COMMENT ON COLUMN template_field_relation.updated_by IS '更新人';

COMMENT ON COLUMN template_field_relation.updated_at IS '更新时间';

COMMENT ON COLUMN template_field_relation.deleted_at IS '删除时间';

COMMENT ON COLUMN template_field_relation.deleted_by IS '删除人';

DROP TABLE IF EXISTS field_option;

CREATE TABLE field_option (
    id SERIAL PRIMARY KEY,
    field_id INT NOT NULL,
    value VARCHAR NOT NULL,
    position INT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW (),
    created_by UUID NOT NULL,
    updated_at TIMESTAMP,
    updated_by UUID,
    deleted_at TIMESTAMP,
    deleted_by UUID
);

-- comments
COMMENT ON COLUMN field_option.id IS '选项值ID';

COMMENT ON COLUMN field_option.field_id IS '所属字段ID';

COMMENT ON COLUMN field_option.value IS '选项值';

COMMENT ON COLUMN field_option.position IS '选项值序列';

COMMENT ON COLUMN field_option.created_at IS '创建时间';

COMMENT ON COLUMN field_option.created_by IS '创建人';

COMMENT ON COLUMN field_option.updated_at IS '更新时间';

COMMENT ON COLUMN field_option.updated_by IS '更新人';

COMMENT ON COLUMN field_option.deleted_at IS '删除时间';

COMMENT ON COLUMN field_option.deleted_by IS '删除人';

-- migrate:down
