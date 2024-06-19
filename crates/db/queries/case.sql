--! get_case_list : (updated_at?, updated_by?, tags?)
SELECT fc.id,
       fc.name,
       fc.module_id,
       fc.template_id,
       fc.tags,
       fc.status,
       fc.created_at,
       (SELECT username FROM users WHERE users.uuid = fc.created_by) AS created_by,
       fc.updated_at,
       (SELECT username FROM users WHERE users.uuid = fc.updated_by) AS updated_by,
       COALESCE(
               (SELECT JSON_AGG(
                               JSON_BUILD_OBJECT(
                                       'id', tcf.id,
                                       'name', tcf.name,
                                       'internal', tcf.internal,
                                       'field_type', tcf.field_type,
                                       'required', tcf.required,
                                       'default_value', tcf.default_value,
                                       'options', COALESCE(
                                               (SELECT JSON_AGG(
                                                               JSON_BUILD_OBJECT(
                                                                       'id', cfo.id,
                                                                       'name', cfo.name,
                                                                       'value', cfo.value,
                                                                       'position', cfo.position
                                                               )
                                                       )
                                                FROM custom_field_option cfo
                                                WHERE cfo.field_id = tcf.id), '[]'
                                                  )
                               )
                       )
                FROM template_custom_field tcf
                WHERE tcf.template_id = fc.template_id), '[]'
       ) AS custom_fields
FROM functional_cases fc
WHERE fc.project_id = :project_id
  AND fc.module_id = ANY(SELECT fm.id FROM file_module fm WHERE fm.id = ANY(:module_id) OR fm.parent_id = ANY(:module_id))
  AND fc.deleted = FALSE
LIMIT :page_size
OFFSET :offset;

--! count
SELECT
    COUNT(*) as total,
    (SELECT name FROM file_module fm WHERE fm.project_id = :project_id) as module_names
FROM functional_cases fc
WHERE project_id = :project_id
AND deleted = FALSE;
