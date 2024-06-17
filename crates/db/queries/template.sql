--! get_template_by_project_id : (description?, updated_at?)
SELECT t.id,
       t.name,
       t.project_id,
       t.description,
       t.internal,
       (SELECT name FROM users WHERE users.uuid = t.created_by) AS created_by,
       t.created_at,
       t.updated_at,
       COALESCE(
               (SELECT JSON_AGG(
                               JSON_BUILD_OBJECT(
                                       'id', tcf.id,
                                       'name', tcf.name,
                                       'field_type' , tcf.field_type,
                                       'internal', tcf.internal,
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
                WHERE tcf.template_id = t.id), '[]'
       )                                                        as custom_fields
FROM template t
WHERE t.project_id = :project_id
  AND t.internal = :internal;


--! get_fields
SELECT cf.id,
       cf.name,
       cf.field_type,
       cf.internal,
       COALESCE(
               (SELECT JSON_AGG(
                               JSON_BUILD_OBJECT(
                                       'id', cfo.id,
                                       'name', cfo.name,
                                       'value', cfo.value,
                                       'position', cfo.position
                               )
                       )
                FROM custom_field_option cfo
                WHERE cfo.field_id = cf.id), '[]'
       ) AS options
FROM custom_field cf
WHERE cf.project_id = :project_id
AND cf.internal = :internal;


--! get_field_option_by_id
SELECT id,
       name,
       value,
       position
FROM custom_field_option
WHERE field_id = :field_id;
