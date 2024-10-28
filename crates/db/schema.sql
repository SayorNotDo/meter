SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: public; Type: SCHEMA; Schema: -; Owner: -
--

-- *not* creating schema, since initdb creates it


--
-- Name: trigger_set_timestamp(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.trigger_set_timestamp() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$;


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: api_permission_relation; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.api_permission_relation (
    id integer NOT NULL,
    uri character varying NOT NULL,
    method character varying NOT NULL,
    permission_id integer NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN api_permission_relation.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.api_permission_relation.id IS '关联关系ID';


--
-- Name: COLUMN api_permission_relation.uri; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.api_permission_relation.uri IS '关联接口';


--
-- Name: COLUMN api_permission_relation.permission_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.api_permission_relation.permission_id IS '权限ID';


--
-- Name: api_permission_relation_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.api_permission_relation_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: api_permission_relation_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.api_permission_relation_id_seq OWNED BY public.api_permission_relation.id;


--
-- Name: case_issue_relation; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.case_issue_relation (
    id integer NOT NULL,
    case_id integer NOT NULL,
    issue_id character varying NOT NULL,
    source character varying NOT NULL,
    uri character varying NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    created_by uuid NOT NULL,
    updated_at timestamp without time zone,
    updated_by uuid
);


--
-- Name: case_issue_relation_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.case_issue_relation_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: case_issue_relation_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.case_issue_relation_id_seq OWNED BY public.case_issue_relation.id;


--
-- Name: custom_field; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.custom_field (
    id integer NOT NULL,
    name character varying NOT NULL,
    field_type character varying NOT NULL,
    project_id integer NOT NULL,
    remark character varying,
    internal boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    created_by uuid NOT NULL,
    updated_at timestamp without time zone,
    updated_by uuid,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone,
    deleted_by uuid
);


--
-- Name: custom_field_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.custom_field_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: custom_field_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.custom_field_id_seq OWNED BY public.custom_field.id;


--
-- Name: custom_field_option; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.custom_field_option (
    id integer NOT NULL,
    field_id integer NOT NULL,
    value character varying NOT NULL,
    "position" integer NOT NULL
);


--
-- Name: COLUMN custom_field_option.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.custom_field_option.id IS '选项值ID';


--
-- Name: COLUMN custom_field_option.field_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.custom_field_option.field_id IS '所属字段ID';


--
-- Name: COLUMN custom_field_option.value; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.custom_field_option.value IS '选项值';


--
-- Name: COLUMN custom_field_option."position"; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.custom_field_option."position" IS '选项值序列';


--
-- Name: custom_field_option_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.custom_field_option_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: custom_field_option_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.custom_field_option_id_seq OWNED BY public.custom_field_option.id;


--
-- Name: element_operation_option; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.element_operation_option (
    id integer NOT NULL,
    option_id integer,
    element_id integer
);


--
-- Name: COLUMN element_operation_option.option_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.element_operation_option.option_id IS '可选操作ID';


--
-- Name: COLUMN element_operation_option.element_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.element_operation_option.element_id IS '元素ID';


--
-- Name: element_operation_option_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.element_operation_option_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: element_operation_option_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.element_operation_option_id_seq OWNED BY public.element_operation_option.id;


--
-- Name: elements; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.elements (
    id integer NOT NULL,
    name character varying NOT NULL,
    module_id integer NOT NULL,
    description character varying,
    type character varying,
    value character varying,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone,
    created_by uuid,
    updated_by uuid,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone,
    deleted_by uuid
);


--
-- Name: COLUMN elements.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.elements.id IS '元素ID';


--
-- Name: COLUMN elements.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.elements.name IS '元素名称';


--
-- Name: COLUMN elements.description; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.elements.description IS '元素描述';


--
-- Name: COLUMN elements.type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.elements.type IS '元素类型';


--
-- Name: COLUMN elements.value; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.elements.value IS '元素值';


--
-- Name: COLUMN elements.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.elements.created_at IS '创建时间';


--
-- Name: COLUMN elements.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.elements.updated_at IS '更新时间';


--
-- Name: COLUMN elements.created_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.elements.created_by IS '创建人';


--
-- Name: COLUMN elements.updated_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.elements.updated_by IS '更新人';


--
-- Name: COLUMN elements.deleted; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.elements.deleted IS '是否删除';


--
-- Name: COLUMN elements.deleted_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.elements.deleted_at IS '删除时间';


--
-- Name: COLUMN elements.deleted_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.elements.deleted_by IS '删除人';


--
-- Name: elements_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.elements_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: elements_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.elements_id_seq OWNED BY public.elements.id;


--
-- Name: environment; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.environment (
    id integer NOT NULL,
    name character varying NOT NULL,
    type character varying NOT NULL,
    internal boolean DEFAULT false NOT NULL,
    dependencies character varying
);


--
-- Name: COLUMN environment.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.environment.id IS '依赖环境ID';


--
-- Name: COLUMN environment.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.environment.name IS '环境名称';


--
-- Name: COLUMN environment.type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.environment.type IS '依赖环境类型';


--
-- Name: COLUMN environment.internal; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.environment.internal IS '是否为内置';


--
-- Name: COLUMN environment.dependencies; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.environment.dependencies IS '依赖';


--
-- Name: environment_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.environment_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: environment_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.environment_id_seq OWNED BY public.environment.id;


--
-- Name: file_module; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.file_module (
    id integer NOT NULL,
    project_id integer NOT NULL,
    name character varying,
    "position" integer DEFAULT 0 NOT NULL,
    module_type character varying NOT NULL,
    attach_info character varying,
    parent_id integer,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    created_by uuid NOT NULL,
    updated_at timestamp without time zone,
    updated_by uuid
);


--
-- Name: COLUMN file_module.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.file_module.id IS '文件管理模块ID';


--
-- Name: COLUMN file_module.project_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.file_module.project_id IS '关联项目ID';


--
-- Name: COLUMN file_module.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.file_module.name IS '文件管理模块名称';


--
-- Name: COLUMN file_module."position"; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.file_module."position" IS '文件管理模块排序标识';


--
-- Name: COLUMN file_module.module_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.file_module.module_type IS '文件管理模块类型：CASE/BUG/PLAN/ELEMENT';


--
-- Name: COLUMN file_module.parent_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.file_module.parent_id IS '文件管理模块父级ID';


--
-- Name: COLUMN file_module.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.file_module.created_at IS '创建时间';


--
-- Name: COLUMN file_module.created_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.file_module.created_by IS '创建人';


--
-- Name: COLUMN file_module.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.file_module.updated_at IS '更新时间';


--
-- Name: COLUMN file_module.updated_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.file_module.updated_by IS '更新人';


--
-- Name: file_module_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.file_module_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: file_module_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.file_module_id_seq OWNED BY public.file_module.id;


--
-- Name: functional_case_custom_field; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.functional_case_custom_field (
    id integer NOT NULL,
    case_id integer NOT NULL,
    field_id integer NOT NULL,
    value character varying NOT NULL
);


--
-- Name: functional_case_custom_field_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.functional_case_custom_field_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: functional_case_custom_field_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.functional_case_custom_field_id_seq OWNED BY public.functional_case_custom_field.id;


--
-- Name: functional_cases; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.functional_cases (
    id integer NOT NULL,
    name character varying NOT NULL,
    module_id integer NOT NULL,
    template_id integer NOT NULL,
    tags character varying,
    status character varying DEFAULT 'UN_REVIEWED'::character varying NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone,
    created_by uuid NOT NULL,
    updated_by uuid,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone,
    deleted_by uuid
);


--
-- Name: COLUMN functional_cases.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.functional_cases.id IS '功能测试用例ID';


--
-- Name: COLUMN functional_cases.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.functional_cases.name IS '功能测试用例名称';


--
-- Name: COLUMN functional_cases.module_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.functional_cases.module_id IS '功能测试用例所属模块ID';


--
-- Name: COLUMN functional_cases.template_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.functional_cases.template_id IS '功能测试用例所属模版ID';


--
-- Name: COLUMN functional_cases.tags; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.functional_cases.tags IS '功能测试用例标签';


--
-- Name: COLUMN functional_cases.status; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.functional_cases.status IS '功能测试用例状态';


--
-- Name: COLUMN functional_cases.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.functional_cases.created_at IS '创建时间';


--
-- Name: COLUMN functional_cases.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.functional_cases.updated_at IS '更新时间';


--
-- Name: COLUMN functional_cases.created_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.functional_cases.created_by IS '创建人';


--
-- Name: COLUMN functional_cases.updated_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.functional_cases.updated_by IS '更新人';


--
-- Name: COLUMN functional_cases.deleted; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.functional_cases.deleted IS '是否删除';


--
-- Name: COLUMN functional_cases.deleted_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.functional_cases.deleted_at IS '删除时间';


--
-- Name: COLUMN functional_cases.deleted_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.functional_cases.deleted_by IS '删除人';


--
-- Name: functional_cases_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.functional_cases_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: functional_cases_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.functional_cases_id_seq OWNED BY public.functional_cases.id;


--
-- Name: machine; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.machine (
    id integer NOT NULL,
    name character varying,
    type character varying,
    addr character varying,
    authentication character varying,
    internal boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    created_by uuid NOT NULL,
    updated_at timestamp without time zone,
    updated_by uuid
);


--
-- Name: machine_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.machine_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: machine_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.machine_id_seq OWNED BY public.machine.id;


--
-- Name: operation_option; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.operation_option (
    id integer NOT NULL,
    name character varying NOT NULL,
    internal boolean DEFAULT false NOT NULL,
    exec character varying NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    created_by uuid NOT NULL
);


--
-- Name: COLUMN operation_option.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.operation_option.id IS '可选操作ID';


--
-- Name: COLUMN operation_option.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.operation_option.name IS '操作名称';


--
-- Name: COLUMN operation_option.exec; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.operation_option.exec IS '执行方式';


--
-- Name: COLUMN operation_option.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.operation_option.created_at IS '创建人';


--
-- Name: COLUMN operation_option.created_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.operation_option.created_by IS '创建时间';


--
-- Name: operation_option_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.operation_option_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: operation_option_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.operation_option_id_seq OWNED BY public.operation_option.id;


--
-- Name: organizations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.organizations (
    id integer NOT NULL,
    name character varying NOT NULL,
    description character varying,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone,
    created_by uuid,
    updated_by uuid,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone,
    deleted_by uuid
);


--
-- Name: COLUMN organizations.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.organizations.id IS '组织ID';


--
-- Name: COLUMN organizations.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.organizations.name IS '组织名称';


--
-- Name: COLUMN organizations.description; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.organizations.description IS '组织描述';


--
-- Name: COLUMN organizations.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.organizations.created_at IS '创建时间';


--
-- Name: COLUMN organizations.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.organizations.updated_at IS '更新时间';


--
-- Name: COLUMN organizations.created_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.organizations.created_by IS '创建人';


--
-- Name: COLUMN organizations.updated_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.organizations.updated_by IS '更新人';


--
-- Name: COLUMN organizations.deleted; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.organizations.deleted IS '是否删除';


--
-- Name: COLUMN organizations.deleted_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.organizations.deleted_at IS '删除时间';


--
-- Name: COLUMN organizations.deleted_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.organizations.deleted_by IS '删除人';


--
-- Name: organizations_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.organizations_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: organizations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.organizations_id_seq OWNED BY public.organizations.id;


--
-- Name: permission; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.permission (
    id integer NOT NULL,
    module character varying NOT NULL,
    scope character varying NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN permission.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.permission.id IS '权限标识ID';


--
-- Name: COLUMN permission.module; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.permission.module IS '所属模块';


--
-- Name: COLUMN permission.scope; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.permission.scope IS '权限控制范围';


--
-- Name: COLUMN permission.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.permission.created_at IS '创建时间';


--
-- Name: permission_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.permission_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: permission_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.permission_id_seq OWNED BY public.permission.id;


--
-- Name: plan_case_relation; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.plan_case_relation (
    id integer NOT NULL,
    plan_id integer NOT NULL,
    case_id integer NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    created_by uuid NOT NULL
);


--
-- Name: COLUMN plan_case_relation.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plan_case_relation.id IS '计划-用例关联关系ID';


--
-- Name: COLUMN plan_case_relation.plan_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plan_case_relation.plan_id IS '测试计划ID';


--
-- Name: COLUMN plan_case_relation.case_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plan_case_relation.case_id IS '测试用例ID';


--
-- Name: COLUMN plan_case_relation.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plan_case_relation.created_at IS '创建时间';


--
-- Name: COLUMN plan_case_relation.created_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plan_case_relation.created_by IS '创建人';


--
-- Name: plan_case_relation_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.plan_case_relation_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: plan_case_relation_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.plan_case_relation_id_seq OWNED BY public.plan_case_relation.id;


--
-- Name: plans; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.plans (
    id integer NOT NULL,
    name character varying NOT NULL,
    project_id integer NOT NULL,
    description character varying,
    module_id integer NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    created_by uuid NOT NULL,
    updated_at timestamp without time zone,
    updated_by uuid,
    status character varying DEFAULT 'NEW'::character varying NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone,
    deleted_by uuid,
    start_date date,
    end_date date
);


--
-- Name: COLUMN plans.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plans.id IS '测试计划ID';


--
-- Name: COLUMN plans.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plans.name IS '测试计划名称';


--
-- Name: COLUMN plans.description; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plans.description IS '测试计划描述';


--
-- Name: COLUMN plans.module_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plans.module_id IS '所属模块ID';


--
-- Name: COLUMN plans.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plans.created_at IS '创建时间';


--
-- Name: COLUMN plans.created_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plans.created_by IS '创建人';


--
-- Name: COLUMN plans.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plans.updated_at IS '更新时间';


--
-- Name: COLUMN plans.updated_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plans.updated_by IS '更新人';


--
-- Name: COLUMN plans.deleted; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plans.deleted IS '是否删除';


--
-- Name: COLUMN plans.deleted_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plans.deleted_at IS '删除时间';


--
-- Name: COLUMN plans.deleted_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plans.deleted_by IS '删除人';


--
-- Name: COLUMN plans.start_date; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plans.start_date IS '起始日期';


--
-- Name: COLUMN plans.end_date; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.plans.end_date IS '结束日期';


--
-- Name: plans_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.plans_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: plans_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.plans_id_seq OWNED BY public.plans.id;


--
-- Name: projects; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.projects (
    id integer NOT NULL,
    name character varying NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone,
    created_by uuid,
    updated_by uuid,
    enable boolean DEFAULT true NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone,
    deleted_by uuid,
    description character varying,
    module_setting character varying
);


--
-- Name: COLUMN projects.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.projects.id IS '项目ID';


--
-- Name: COLUMN projects.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.projects.name IS '项目名称';


--
-- Name: COLUMN projects.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.projects.created_at IS '创建时间';


--
-- Name: COLUMN projects.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.projects.updated_at IS '更新时间';


--
-- Name: COLUMN projects.created_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.projects.created_by IS '创建人';


--
-- Name: COLUMN projects.updated_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.projects.updated_by IS '更新人';


--
-- Name: COLUMN projects.deleted; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.projects.deleted IS '是否删除';


--
-- Name: COLUMN projects.deleted_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.projects.deleted_at IS '删除时间';


--
-- Name: COLUMN projects.deleted_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.projects.deleted_by IS '删除人';


--
-- Name: COLUMN projects.description; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.projects.description IS '项目描述';


--
-- Name: COLUMN projects.module_setting; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.projects.module_setting IS '模块设置';


--
-- Name: projects_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.projects_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: projects_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.projects_id_seq OWNED BY public.projects.id;


--
-- Name: role_permission_relation; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.role_permission_relation (
    id integer NOT NULL,
    role_id integer NOT NULL,
    permission_id integer NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL
);


--
-- Name: COLUMN role_permission_relation.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.role_permission_relation.id IS '关联关系ID';


--
-- Name: COLUMN role_permission_relation.role_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.role_permission_relation.role_id IS '角色ID';


--
-- Name: COLUMN role_permission_relation.permission_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.role_permission_relation.permission_id IS '权限ID';


--
-- Name: role_permission_relation_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.role_permission_relation_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: role_permission_relation_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.role_permission_relation_id_seq OWNED BY public.role_permission_relation.id;


--
-- Name: schema_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.schema_migrations (
    version character varying(128) NOT NULL
);


--
-- Name: script; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.script (
    id integer NOT NULL,
    case_id integer NOT NULL,
    environment character varying NOT NULL,
    path character varying NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    created_by uuid NOT NULL,
    updated_at timestamp without time zone,
    updated_by uuid
);


--
-- Name: script_element_relation; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.script_element_relation (
    id integer NOT NULL,
    script_id integer NOT NULL,
    field_type character varying NOT NULL,
    element_operation_id integer NOT NULL,
    "position" integer NOT NULL,
    attach_info character varying
);


--
-- Name: script_element_relation_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.script_element_relation_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: script_element_relation_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.script_element_relation_id_seq OWNED BY public.script_element_relation.id;


--
-- Name: script_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.script_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: script_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.script_id_seq OWNED BY public.script.id;


--
-- Name: template; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.template (
    id integer NOT NULL,
    name character varying NOT NULL,
    project_id integer NOT NULL,
    description character varying,
    internal boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    created_by uuid NOT NULL,
    updated_at timestamp without time zone,
    updated_by uuid,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone,
    deleted_by uuid
);


--
-- Name: COLUMN template.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.template.id IS '模板ID';


--
-- Name: COLUMN template.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.template.name IS '模板名称';


--
-- Name: COLUMN template.description; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.template.description IS '模板描述';


--
-- Name: COLUMN template.internal; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.template.internal IS '是否内置模板';


--
-- Name: COLUMN template.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.template.created_at IS '创建时间';


--
-- Name: COLUMN template.created_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.template.created_by IS '创建人';


--
-- Name: COLUMN template.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.template.updated_at IS '更新时间';


--
-- Name: COLUMN template.updated_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.template.updated_by IS '更新人';


--
-- Name: COLUMN template.deleted; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.template.deleted IS '是否删除';


--
-- Name: COLUMN template.deleted_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.template.deleted_at IS '删除时间';


--
-- Name: COLUMN template.deleted_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.template.deleted_by IS '删除人';


--
-- Name: template_custom_field; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.template_custom_field (
    id integer NOT NULL,
    template_id integer NOT NULL,
    name character varying NOT NULL,
    required boolean DEFAULT false NOT NULL,
    field_type character varying NOT NULL,
    remark character varying,
    default_value character varying,
    internal boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    created_by uuid NOT NULL,
    updated_at timestamp without time zone,
    updated_by uuid,
    deleted boolean DEFAULT false NOT NULL,
    deleted_at timestamp without time zone,
    deleted_by uuid
);


--
-- Name: COLUMN template_custom_field.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.template_custom_field.id IS '模版自定义字段ID';


--
-- Name: COLUMN template_custom_field.template_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.template_custom_field.template_id IS '关联模版ID';


--
-- Name: COLUMN template_custom_field.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.template_custom_field.name IS '模版名称';


--
-- Name: COLUMN template_custom_field.required; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.template_custom_field.required IS '是否为必要字段';


--
-- Name: template_custom_field_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.template_custom_field_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: template_custom_field_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.template_custom_field_id_seq OWNED BY public.template_custom_field.id;


--
-- Name: template_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.template_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: template_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.template_id_seq OWNED BY public.template.id;


--
-- Name: user_role; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_role (
    id integer NOT NULL,
    name character varying NOT NULL,
    type character varying NOT NULL,
    internal boolean DEFAULT false NOT NULL,
    description character varying,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    created_by uuid,
    updated_at timestamp without time zone
);


--
-- Name: COLUMN user_role.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_role.id IS '角色ID';


--
-- Name: COLUMN user_role.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_role.name IS '角色名称';


--
-- Name: COLUMN user_role.type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_role.type IS '所属类型 SYSTEM, ORGANIZATION, PROJECT';


--
-- Name: COLUMN user_role.internal; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_role.internal IS '是否内置角色';


--
-- Name: COLUMN user_role.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_role.created_at IS '创建时间';


--
-- Name: COLUMN user_role.created_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_role.created_by IS '创建人';


--
-- Name: COLUMN user_role.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_role.updated_at IS '更新时间';


--
-- Name: user_role_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_role_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_role_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_role_id_seq OWNED BY public.user_role.id;


--
-- Name: user_role_relation; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.user_role_relation (
    id integer NOT NULL,
    user_id uuid,
    role_id integer,
    project_id integer NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    created_by uuid,
    updated_at timestamp without time zone,
    updated_by uuid
);


--
-- Name: COLUMN user_role_relation.user_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_role_relation.user_id IS '用户ID';


--
-- Name: COLUMN user_role_relation.role_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_role_relation.role_id IS '角色ID';


--
-- Name: COLUMN user_role_relation.project_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_role_relation.project_id IS '关联项目ID';


--
-- Name: COLUMN user_role_relation.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_role_relation.created_at IS '创建时间';


--
-- Name: COLUMN user_role_relation.created_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_role_relation.created_by IS '创建人';


--
-- Name: COLUMN user_role_relation.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_role_relation.updated_at IS '更新时间';


--
-- Name: COLUMN user_role_relation.updated_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.user_role_relation.updated_by IS '更新人';


--
-- Name: user_role_relation_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_role_relation_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_role_relation_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_role_relation_id_seq OWNED BY public.user_role_relation.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.users (
    id integer NOT NULL,
    uuid uuid NOT NULL,
    username character varying NOT NULL,
    hashed_password character varying NOT NULL,
    email character varying,
    enable boolean DEFAULT false,
    last_project_id integer,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone,
    deleted_at timestamp without time zone,
    deleted_by uuid
);


--
-- Name: COLUMN users.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.id IS '用户ID';


--
-- Name: COLUMN users.uuid; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.uuid IS '用户唯一标识';


--
-- Name: COLUMN users.username; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.username IS '用户名';


--
-- Name: COLUMN users.hashed_password; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.hashed_password IS '用户密码';


--
-- Name: COLUMN users.email; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.email IS '用户邮箱';


--
-- Name: COLUMN users.last_project_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.last_project_id IS '最后登录的项目ID';


--
-- Name: COLUMN users.created_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.created_at IS '创建时间';


--
-- Name: COLUMN users.updated_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.updated_at IS '更新时间';


--
-- Name: COLUMN users.deleted_at; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.deleted_at IS '删除时间';


--
-- Name: COLUMN users.deleted_by; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.users.deleted_by IS '删除执行人';


--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: api_permission_relation id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.api_permission_relation ALTER COLUMN id SET DEFAULT nextval('public.api_permission_relation_id_seq'::regclass);


--
-- Name: case_issue_relation id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.case_issue_relation ALTER COLUMN id SET DEFAULT nextval('public.case_issue_relation_id_seq'::regclass);


--
-- Name: custom_field id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.custom_field ALTER COLUMN id SET DEFAULT nextval('public.custom_field_id_seq'::regclass);


--
-- Name: custom_field_option id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.custom_field_option ALTER COLUMN id SET DEFAULT nextval('public.custom_field_option_id_seq'::regclass);


--
-- Name: element_operation_option id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.element_operation_option ALTER COLUMN id SET DEFAULT nextval('public.element_operation_option_id_seq'::regclass);


--
-- Name: elements id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.elements ALTER COLUMN id SET DEFAULT nextval('public.elements_id_seq'::regclass);


--
-- Name: environment id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.environment ALTER COLUMN id SET DEFAULT nextval('public.environment_id_seq'::regclass);


--
-- Name: file_module id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.file_module ALTER COLUMN id SET DEFAULT nextval('public.file_module_id_seq'::regclass);


--
-- Name: functional_case_custom_field id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.functional_case_custom_field ALTER COLUMN id SET DEFAULT nextval('public.functional_case_custom_field_id_seq'::regclass);


--
-- Name: functional_cases id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.functional_cases ALTER COLUMN id SET DEFAULT nextval('public.functional_cases_id_seq'::regclass);


--
-- Name: machine id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.machine ALTER COLUMN id SET DEFAULT nextval('public.machine_id_seq'::regclass);


--
-- Name: operation_option id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.operation_option ALTER COLUMN id SET DEFAULT nextval('public.operation_option_id_seq'::regclass);


--
-- Name: organizations id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.organizations ALTER COLUMN id SET DEFAULT nextval('public.organizations_id_seq'::regclass);


--
-- Name: permission id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.permission ALTER COLUMN id SET DEFAULT nextval('public.permission_id_seq'::regclass);


--
-- Name: plan_case_relation id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.plan_case_relation ALTER COLUMN id SET DEFAULT nextval('public.plan_case_relation_id_seq'::regclass);


--
-- Name: plans id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.plans ALTER COLUMN id SET DEFAULT nextval('public.plans_id_seq'::regclass);


--
-- Name: projects id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.projects ALTER COLUMN id SET DEFAULT nextval('public.projects_id_seq'::regclass);


--
-- Name: role_permission_relation id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.role_permission_relation ALTER COLUMN id SET DEFAULT nextval('public.role_permission_relation_id_seq'::regclass);


--
-- Name: script id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.script ALTER COLUMN id SET DEFAULT nextval('public.script_id_seq'::regclass);


--
-- Name: script_element_relation id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.script_element_relation ALTER COLUMN id SET DEFAULT nextval('public.script_element_relation_id_seq'::regclass);


--
-- Name: template id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.template ALTER COLUMN id SET DEFAULT nextval('public.template_id_seq'::regclass);


--
-- Name: template_custom_field id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.template_custom_field ALTER COLUMN id SET DEFAULT nextval('public.template_custom_field_id_seq'::regclass);


--
-- Name: user_role id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_role ALTER COLUMN id SET DEFAULT nextval('public.user_role_id_seq'::regclass);


--
-- Name: user_role_relation id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_role_relation ALTER COLUMN id SET DEFAULT nextval('public.user_role_relation_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: api_permission_relation api_permission_relation_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.api_permission_relation
    ADD CONSTRAINT api_permission_relation_pkey PRIMARY KEY (id);


--
-- Name: case_issue_relation case_issue_relation_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.case_issue_relation
    ADD CONSTRAINT case_issue_relation_pkey PRIMARY KEY (id);


--
-- Name: custom_field_option custom_field_option_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.custom_field_option
    ADD CONSTRAINT custom_field_option_pkey PRIMARY KEY (id);


--
-- Name: custom_field custom_field_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.custom_field
    ADD CONSTRAINT custom_field_pkey PRIMARY KEY (id);


--
-- Name: element_operation_option element_operation_option_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.element_operation_option
    ADD CONSTRAINT element_operation_option_pkey PRIMARY KEY (id);


--
-- Name: elements elements_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.elements
    ADD CONSTRAINT elements_pkey PRIMARY KEY (id);


--
-- Name: environment environment_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.environment
    ADD CONSTRAINT environment_pkey PRIMARY KEY (id);


--
-- Name: file_module file_module_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.file_module
    ADD CONSTRAINT file_module_pkey PRIMARY KEY (id);


--
-- Name: functional_case_custom_field functional_case_custom_field_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.functional_case_custom_field
    ADD CONSTRAINT functional_case_custom_field_pkey PRIMARY KEY (id);


--
-- Name: functional_cases functional_cases_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.functional_cases
    ADD CONSTRAINT functional_cases_pkey PRIMARY KEY (id);


--
-- Name: machine machine_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.machine
    ADD CONSTRAINT machine_pkey PRIMARY KEY (id);


--
-- Name: operation_option operation_option_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.operation_option
    ADD CONSTRAINT operation_option_pkey PRIMARY KEY (id);


--
-- Name: organizations organizations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.organizations
    ADD CONSTRAINT organizations_pkey PRIMARY KEY (id);


--
-- Name: permission permission_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.permission
    ADD CONSTRAINT permission_pkey PRIMARY KEY (id);


--
-- Name: plan_case_relation plan_case_relation_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.plan_case_relation
    ADD CONSTRAINT plan_case_relation_pkey PRIMARY KEY (id);


--
-- Name: plans plans_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.plans
    ADD CONSTRAINT plans_pkey PRIMARY KEY (id);


--
-- Name: projects projects_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_pkey PRIMARY KEY (id);


--
-- Name: role_permission_relation role_permission_relation_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.role_permission_relation
    ADD CONSTRAINT role_permission_relation_pkey PRIMARY KEY (id);


--
-- Name: schema_migrations schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.schema_migrations
    ADD CONSTRAINT schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: script_element_relation script_element_relation_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.script_element_relation
    ADD CONSTRAINT script_element_relation_pkey PRIMARY KEY (id);


--
-- Name: script script_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.script
    ADD CONSTRAINT script_pkey PRIMARY KEY (id);


--
-- Name: template_custom_field template_custom_field_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.template_custom_field
    ADD CONSTRAINT template_custom_field_pkey PRIMARY KEY (id);


--
-- Name: template template_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.template
    ADD CONSTRAINT template_pkey PRIMARY KEY (id);


--
-- Name: user_role user_role_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_role
    ADD CONSTRAINT user_role_pkey PRIMARY KEY (id);


--
-- Name: user_role_relation user_role_relation_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.user_role_relation
    ADD CONSTRAINT user_role_relation_pkey PRIMARY KEY (id);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: users users_username_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_username_key UNIQUE (username);


--
-- Name: case_issue_relation set_timestamp_case_issue_relation; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_timestamp_case_issue_relation BEFORE UPDATE ON public.case_issue_relation FOR EACH ROW EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- Name: elements set_timestamp_elements; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_timestamp_elements BEFORE UPDATE ON public.elements FOR EACH ROW EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- Name: file_module set_timestamp_file_module; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_timestamp_file_module BEFORE UPDATE ON public.file_module FOR EACH ROW EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- Name: functional_cases set_timestamp_functional_cases; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_timestamp_functional_cases BEFORE UPDATE ON public.functional_cases FOR EACH ROW EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- Name: machine set_timestamp_machine; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_timestamp_machine BEFORE UPDATE ON public.machine FOR EACH ROW EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- Name: organizations set_timestamp_organization; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_timestamp_organization BEFORE UPDATE ON public.organizations FOR EACH ROW EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- Name: plans set_timestamp_plan; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_timestamp_plan BEFORE UPDATE ON public.plans FOR EACH ROW EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- Name: projects set_timestamp_project; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_timestamp_project BEFORE UPDATE ON public.projects FOR EACH ROW EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- Name: custom_field set_timestamp_template; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_timestamp_template BEFORE UPDATE ON public.custom_field FOR EACH ROW EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- Name: template set_timestamp_template; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_timestamp_template BEFORE UPDATE ON public.template FOR EACH ROW EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- Name: template_custom_field set_timestamp_template_custom_field; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_timestamp_template_custom_field BEFORE UPDATE ON public.template_custom_field FOR EACH ROW EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- Name: users set_timestamp_user; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_timestamp_user BEFORE UPDATE ON public.users FOR EACH ROW EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- Name: user_role set_timestamp_user_role; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_timestamp_user_role BEFORE UPDATE ON public.user_role FOR EACH ROW EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- Name: user_role_relation set_timestamp_user_role; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_timestamp_user_role BEFORE UPDATE ON public.user_role_relation FOR EACH ROW EXECUTE FUNCTION public.trigger_set_timestamp();


--
-- PostgreSQL database dump complete
--


--
-- Dbmate schema migrations
--

INSERT INTO public.schema_migrations (version) VALUES
    ('20240521174327'),
    ('20240601150319'),
    ('20240612101614'),
    ('20240614033050');
