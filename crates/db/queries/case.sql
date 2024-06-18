--! get_case_list
SELECT fc.id,
       fc.name,
       fc.module_id,
       fc.template_id,
       fc.tags,
       fc.status,
       fc.created_at,
       fc.created_by,
       fc.updated_at,
       fc.updated_by,
       COALESCE(
               (SELECT JSON_AGG(
                               JSON_BUILD_OBJECT(
                                       'id', tcf.id,
                                       'name', tcf.name,
                                       'internal', tcf.internal,
                                       'default_value', tcf.default_value,
                                       'options', COALESCE(
                                               (SELECT JSON_AGG(
                                                               JSON_BUILD_OBJECT(
                                                                       'id', cfo.id,
                                                                       'name', cfo.name,
                                                                       'value', cfo.value,
                                                                       'pos', cfo.position
                                                               )
                                                       )
                                                FROM custom_field_option cfo
                                                WHERE cfo.field_id = tcf.id), '[]'
                                                  )
                               )
                       )
                FROM template_custom_field tcf
                WHERE tcf.id = fc.template_id), '[]'
       ) AS custom_fields
FROM functional_cases fc
WHERE project_id = :project_id
  AND deleted = FALSE
LIMIT :page_size
OFFSET :offset;

--! count
SELECT
    COUNT(*)
FROM functional_cases fc
WHERE project_id = :project_id
AND deleted = FALSE;
