-- migrate:up

DROP TABLE IF EXISTS projects;

CREATE TABLE projects
(
    id              SERIAL PRIMARY KEY,
    name            VARCHAR   NOT NULL,
    organization_id INT       NOT NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMP,
    created_by      UUID,
    updated_by      UUID,
    enable          BOOLEAN   NOT NULL DEFAULT TRUE,
    deleted         BOOLEAN   NOT NULL DEFAULT FALSE,
    deleted_at      TIMESTAMP,
    deleted_by      UUID,
    description     VARCHAR,
    module_setting  VARCHAR
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
CREATE TRIGGER set_timestamp_project
    BEFORE UPDATE
    ON projects
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

DROP TABLE IF EXISTS organizations;

CREATE TABLE organizations
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR   NOT NULL,
    description VARCHAR,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMP,
    created_by  UUID,
    updated_by  UUID,
    deleted     BOOLEAN   NOT NULL DEFAULT FALSE,
    deleted_at  TIMESTAMP,
    deleted_by  UUID
);


-- comments
COMMENT ON COLUMN organizations.id IS '组织ID';
COMMENT ON COLUMN organizations.name IS '组织名称';
COMMENT ON COLUMN organizations.description IS '组织描述';
COMMENT ON COLUMN organizations.created_at IS '创建时间';
COMMENT ON COLUMN organizations.updated_at IS '更新时间';
COMMENT ON COLUMN organizations.created_by IS '创建人';
COMMENT ON COLUMN organizations.updated_by IS '更新人';
COMMENT ON COLUMN organizations.deleted IS '是否删除';
COMMENT ON COLUMN organizations.deleted_at IS '删除时间';
COMMENT ON COLUMN organizations.deleted_by IS '删除人';

-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp_organization
    BEFORE UPDATE
    ON organizations
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

DROP TABLE IF EXISTS functional_cases;

CREATE TABLE functional_cases
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR   NOT NULL,
    module_id  INT       NOT NULL,
    template_id INT NOT NULL,
    tags       VARCHAR,
    status     VARCHAR       NOT NULL DEFAULT 'UN_REVIEWED',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP,
    created_by UUID NOT NULL,
    updated_by UUID,
    deleted    BOOLEAN   NOT NULL DEFAULT FALSE,
    deleted_at TIMESTAMP,
    deleted_by UUID
);

-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp_functional_cases
    BEFORE UPDATE
    ON functional_cases
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- comments
COMMENT ON COLUMN functional_cases.id IS '功能测试用例ID';
COMMENT ON COLUMN functional_cases.name IS '功能测试用例名称';
COMMENT ON COLUMN functional_cases.module_id IS '功能测试用例所属模块ID';
COMMENT ON COLUMN functional_cases.template_id IS '功能测试用例所属模版ID';
COMMENT ON COLUMN functional_cases.tags IS '功能测试用例标签';
COMMENT ON COLUMN functional_cases.status IS '功能测试用例状态';
COMMENT ON COLUMN functional_cases.created_at IS '创建时间';
COMMENT ON COLUMN functional_cases.updated_at IS '更新时间';
COMMENT ON COLUMN functional_cases.created_by IS '创建人';
COMMENT ON COLUMN functional_cases.updated_by IS '更新人';
COMMENT ON COLUMN functional_cases.deleted IS '是否删除';
COMMENT ON COLUMN functional_cases.deleted_at IS '删除时间';
COMMENT ON COLUMN functional_cases.deleted_by IS '删除人';

DROP TABLE IF EXISTS functional_case_custom_field;

CREATE TABLE functional_case_custom_field (
    id SERIAL PRIMARY KEY,
    case_id INT NOT NULL,
    field_id INT NOT NULL,
    value   VARCHAR NOT NULL
);

DROP TABLE IF EXISTS case_issue_relation;

CREATE TABLE case_issue_relation (
    id SERIAL PRIMARY KEY,
    case_id INT NOT NULL,
    issue_id VARCHAR NOT NULL,
    source  VARCHAR NOT NULL,
    uri VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL,
    updated_at TIMESTAMP,
    updated_by UUID
);

-- create trigger: set updated_at field
CREATE TRIGGER set_timestamp_case_issue_relation
    BEFORE UPDATE
    ON case_issue_relation
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

DROP TABLE IF EXISTS script_element_relation;

CREATE TABLE script_element_relation (
    id                      SERIAL PRIMARY KEY,
    script_id               INT NOT NULL,
    field_type              VARCHAR NOT NULL,
    element_operation_id    INT NOT NULL,
    position                INT NOT NULL,
    attach_info             VARCHAR
);


DROP TABLE IF EXISTS machine;

CREATE TABLE machine
(
    id SERIAL PRIMARY KEY,
    name VARCHAR,
    type VARCHAR,
    addr VARCHAR,
    authentication VARCHAR,
    internal BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL,
    updated_at TIMESTAMP,
    updated_by UUID
)

--create trigger: set updated_at field
CREATE TRIGGER set_timestamp_machine
    BEFORE UPDATE
    ON machine
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();


DROP TABLE IF EXISTS file_module;

CREATE TABLE file_module
(
    id          SERIAL PRIMARY KEY,
    project_id  INT       NOT NULL,
    name        VARCHAR,
    position    INT       NOT NULL DEFAULT 0,
    module_type VARCHAR NOT NULL,
    attach_info VARCHAR,
    parent_id   INT,
    created_at  TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by  UUID      NOT NULL,
    updated_at  TIMESTAMP,
    updated_by  UUID
);

--create trigger: set updated_at field
CREATE TRIGGER set_timestamp_file_module
    BEFORE UPDATE
    ON file_module
    FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- comments
COMMENT ON COLUMN file_module.id IS '文件管理模块ID';
COMMENT ON COLUMN file_module.project_id IS '关联项目ID';
COMMENT ON COLUMN file_module.name IS '文件管理模块名称';
COMMENT ON COLUMN file_module.position IS '文件管理模块排序标识';
COMMENT ON COLUMN file_module.module_type IS '文件管理模块类型：CASE/BUG/PLAN/ELEMENT';
COMMENT ON COLUMN file_module.parent_id IS '文件管理模块父级ID';
COMMENT ON COLUMN file_module.created_at IS '创建时间';
COMMENT ON COLUMN file_module.updated_at IS '更新时间';
COMMENT ON COLUMN file_module.created_by IS '创建人';
COMMENT ON COLUMN file_module.updated_by IS '更新人';


-- migrate:down
