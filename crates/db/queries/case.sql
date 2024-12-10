--! insert_functional_case
INSERT INTO functional_cases
(
    name,
    module_id,
    template_id,
    tags,
    created_by
) VALUES (
  :name,
  :module_id,
  :template_id,
  :tags,
  :created_by
) RETURNING id;

--! insert_case_field_relation
INSERT INTO functional_case_field_relation (
    case_id,
    field_id,
    field_value,
    created_by
) VALUES (
    :case_id,
    :field_id,
    :value,
    :created_by
) RETURNING id;

--! insert_case_execute_record
INSERT INTO functional_case_execute_record (
    case_id,
    result,
    attach_info,
    created_by
) VALUES (
    :case_id,
    :result,
    :attach_info,
    :created_by
) RETURNING id;

--! get_last_execute_record_by_case_id : (updated_at?, updated_by?)
SELECT  fcer.id,
        fcer.case_id,
        fcer.result,
        fcer.attach_info,
        fcer.created_at,
        uc.username AS created_by,
        fcer.updated_at,
        uu.username AS updated_by
FROM functional_case_execute_record fcer
LEFT JOIN users uc ON uc.uuid = fcer.created_by
LEFT JOIN users uu ON uu.uuid = fcer.updated_by
WHERE case_id = :case_id
ORDER BY fcer.created_at
DESC LIMIT 1;

--! insert_case_issue_relation
INSERT INTO case_issue_relation (
    case_id,
    issue_id,
    source,
    uri,
    created_by
) VALUES (
    :case_id,
    :field_id,
    :source,
    :uri,
    :created_by
);

--! get_fields_by_case_id : (remark?, options?)
SELECT fcfr.id,
       f.name,
       f.project_id,
       fcfr.field_id,
       f.field_type,
       f.remark,
       fcfr.field_value,
       (SELECT JSON_AGG(JSON_BUILD_OBJECT(
            'id', fo.id,
            'field_id', fo.field_id,
            'value', fo.value,
            'position', fo.position
       )) FROM  field_option fo WHERE fo.field_id = fcfr.field_id) AS options,
       f.internal,
       tfr.required
FROM functional_case_field_relation fcfr
LEFT JOIN field f ON f.id = fcfr.field_id
LEFT JOIN template_field_relation tfr ON tfr.field_id = fcfr.field_id
WHERE fcfr.case_id = :case_id;

--! get_functional_case_list : (updated_at?, updated_by?, attach_info?)
SELECT fc.id,
       fc.name,
       fc.template_id,
       JSON_BUILD_OBJECT(
        'id', fm.id,
        'name', fm.name,
        'position', fm.position,
        'module_type', fm.module_type,
        'parent_id', fm.parent_id
       ) AS module,
       fc.tags,
       fc.attach_info,
       fc.status,
       fc.edit_type,
       fc.created_at,
       (SELECT username FROM users WHERE users.uuid = fc.created_by) AS created_by,
       fc.updated_at,
       (SELECT username FROM users WHERE users.uuid = fc.updated_by) AS updated_by
FROM functional_cases fc
LEFT JOIN file_module fm ON fc.module_id = fm.id
WHERE
(fc.module_id = ANY(:module_id) OR fm.parent_id = ANY(:module_id)) AND fc.deleted_at IS NULL AND fc.deleted_by IS NULL
LIMIT :page_size
OFFSET :offset;

--! count
SELECT
    fm.name AS module_name,
    COUNT(fc.id) AS case_count
FROM file_module fm
LEFT JOIN functional_cases fc
    ON fc.module_id = fm.id
WHERE
    fm.project_id = :project_id
GROUP BY fm.name;


--! count_deleted_case
SELECT
    fm.name AS module_name,
    COUNT(fc.id) AS case_count
FROM file_module fm
LEFT JOIN functional_cases fc
    ON fc.module_id = fm.id
WHERE
    fm.project_id = :project_id AND fc.deleted_by IS NOT NULL
GROUP BY fm.name;

--! count_case
SELECT COUNT(*)
FROM functional_cases
WHERE module_id IN (SELECT id FROM file_module WHERE project_id = :project_id)
AND deleted_at IS NULL AND deleted_by IS NULL;

--! count_by_module_id
SELECT
    COUNT(fc.id) AS count
FROM functional_cases fc
WHERE
    fc.module_id = :module_id;

--! update_functional_case (updated_by?)
UPDATE functional_cases
    SET name = :name,
        tags = :tags,
        module_id = :module_id,
        status = :status,
        updated_by = :updated_by,
        updated_at = NOW()
WHERE id = :case_id;

--! soft_delete_functional_case
UPDATE functional_cases
SET deleted_at = NOW(),
    deleted_by = :deleted_by,
    updated_by = :deleted_by
WHERE id = :case_id;

--! soft_delete_functional_case_by_module_id
WITH case_soft_delete AS (UPDATE functional_cases
SET deleted_at = NOW(),
    deleted_by = :deleted_by,
    updated_by = :deleted_by
WHERE module_id = :module_id
RETURNING id)
UPDATE functional_case_field_relation
SET deleted_at = NOW(),
    deleted_by = :deleted_by,
    updated_by = :deleted_by
WHERE case_id IN (SELECT id FROM case_soft_delete);

--! get_functional_case_by_name : (attach_info?, updated_at?, updated_by?)
SELECT
    fc.id,
    fc.name,
    fc.tags,
    fc.template_id,
    JSON_BUILD_OBJECT(
        'id', fm.id,
        'name', fm.name,
        'position', fm.position,
        'module_type', fm.module_type,
        'parent_id', fm.parent_id
    ) AS module,
    fc.attach_info,
    fc.status,
    fc.edit_type,
    fc.created_at,
    c.username AS created_by,
    fc.updated_at,
    u.username AS updated_by
FROM functional_cases fc
LEFT JOIN file_module fm
    ON fm.id = fc.module_id
LEFT JOIN users c
    ON c.uuid = fc.created_by
LEFT JOIN users u
    ON u.uuid = fc.updated_by
WHERE
    fc.name = :case_name;

--! get_functional_case_by_id : (attach_info?, updated_at?, updated_by?)
SELECT
    fc.id,
    fc.name,
    fc.tags,
    fc.template_id,
    JSON_BUILD_OBJECT(
        'id', fm.id,
        'name', fm.name,
        'position', fm.position,
        'module_type', fm.module_type,
        'parent_id', fm.parent_id
    ) AS module,
    fc.attach_info,
    fc.status,
    fc.edit_type,
    fc.created_at,
    u.username AS created_by,
    fc.updated_at,
    (SELECT username FROM users WHERE users.uuid = fc.updated_by) AS updated_by
FROM functional_cases fc
INNER JOIN file_module fm
    ON fm.id = fc.module_id
INNER JOIN users u
    ON u.uuid = fc.created_by
WHERE
    fc.id = :case_id;

--! insert_script
INSERT INTO script
(
    case_id,
    environment,
    path,
    created_by
) VALUES (
    :case_id,
    :environment,
    :path,
    :created_by
) RETURNING id;

--! insert_script_element_relation
INSERT INTO script_element_relation(
    script_id,
    field_type,
    element_operation_id,
    position,
    attach_info
) VALUES (
    :script_id,
    :field_type,
    (SELECT id FROM element_operation_option WHERE option_id = :option_id AND element_id = :element_id),
    :position,
    :attach_info
) RETURNING id;

--! get_machine : (updated_by?, updated_at?)
SELECT
    name,
    addr,
    type,
    authentication,
    internal,
    created_by,
    created_at,
    updated_by,
    updated_at
FROM machine
WHERE id = :id;

--! delete_by_module_id
UPDATE functional_cases
SET deleted_at = NOW(), deleted_by = :deleted_by
WHERE module_id = :module_id;
