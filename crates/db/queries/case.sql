--! insert_functional_case (tags?)
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

--! insert_case_field_relation (value?, option_id?) :
INSERT INTO functional_case_field_relation (
    case_id,
    field_id,
    value,
    option_id
) VALUES (
    :case_id,
    :field_id,
    :value,
    :option_id
) RETURNING id;

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

--! get_case_list : (updated_at?, updated_by?, tags?)
SELECT fc.id,
       fc.name,
       fc.template_id,
       (SELECT name FROM file_module WHERE file_module.id = fc.module_id) AS module_name,
       fc.tags,
       fc.status,
       fc.created_at,
       (SELECT username FROM users WHERE users.uuid = fc.created_by) AS created_by,
       fc.updated_at,
       (SELECT username FROM users WHERE users.uuid = fc.updated_by) AS updated_by,
       COALESCE(
               (SELECT JSON_AGG(
                               JSON_BUILD_OBJECT(
                                       'id', tfr.id,
                                       'name', f.name,
                                       'internal', f.internal,
                                       'field_type', f.field_type,
                                       'required', tfr.required,
                                       'default_value', tfr.default_value,
                                       'options', COALESCE(
                                               (SELECT JSON_AGG(
                                                               JSON_BUILD_OBJECT(
                                                                       'id', fo.id,
                                                                       'value', fo.value,
                                                                       'position', fo.position
                                                               )
                                                       )
                                                FROM field_option fo
                                                WHERE fo.field_id = f.id), '[]'
                                                  )
                               )
                       )
                FROM template_field_relation tfr
                LEFT JOIN field f ON tfr.field_id = f.id
                WHERE tfr.template_id = fc.template_id), '[]'
       ) AS fields
FROM functional_cases fc
WHERE fc.module_id = ANY(SELECT fm.id FROM file_module fm WHERE fm.id = ANY(:module_id) OR fm.parent_id = ANY(:module_id))
  AND fc.deleted_at IS NULL AND fc.deleted_by IS NULL
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

--! count_by_module_id
SELECT
    COUNT(fc.id) AS count
FROM functional_cases fc
WHERE
    fc.module_id = :module_id;


--! detail : (attach_info?, tags?, updated_at?, updated_by?)
SELECT
    fc.id,
    fc.name,
    fc.tags,
    fc.template_id,
    fm.name AS module_name,
    fm.attach_info,
    COALESCE(
            (SELECT JSON_AGG(
                            JSON_BUILD_OBJECT(
                                    'id', tfr.id,
                                    'name', f.name,
                                    'internal', f.internal,
                                    'field_type', f.field_type,
                                    'required', tfr.required,
                                    'default_value', tfr.default_value,
                                    'options', COALESCE(
                                            (SELECT JSON_AGG(
                                                            JSON_BUILD_OBJECT(
                                                                    'id', fo.id,
                                                                    'value', fo.value,
                                                                    'position', fo.position
                                                            )
                                                    )
                                             FROM field_option fo
                                             WHERE fo.field_id = tfr.id), '[]'
                                               )
                            )
                    )
             FROM template_field_relation tfr
             LEFT JOIN field f ON tfr.field_id = f.id
             WHERE tfr.template_id = fc.template_id), '[]'
    ) AS fields,
    fc.status,
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
