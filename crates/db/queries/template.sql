--! get_template_by_id : (description?, updated_at?)
SELECT
    t.id,
    t.name,
    t.project_id,
    t.description,
    t.internal,
    (SELECT username FROM users WHERE users.uuid = t.created_by) AS created_by,
    t.created_at,
    t.updated_at,
    COALESCE(
        (SELECT JSON_AGG(
                    JSON_BUILD_OBJECT(
                        'id', tfr.id,
                        'name', f.name,
                        'field_type', f.field_type,
                        'internal', f.internal,
                        'required', tfr.required,
                        'default_value', tfr.default_value,
                        'options', COALESCE(
                                    (SELECT JSON_AGG(
                                        JSON_BUILD_OBJECT(
                                            'id', fo.id,
                                            'value', fo.value,
                                            'position', fo.position
                                        )
                                    ) FROM field_option fo
                                    WHERE fo.field_id = tfr.id), '[]'
                    )
                    )
        ) FROM template_field_relation tfr
        LEFT JOIN field f ON tfr.field_id = f.id
        WHERE tfr.template_id = t.id), '[]'
    ) AS fields
FROM template t
WHERE t.id = :template_id;


--! get_template_by_project_id : (description?, updated_at?)
SELECT t.id,
       t.name,
       t.project_id,
       t.description,
       t.internal,
       (SELECT username FROM users WHERE users.uuid = t.created_by) AS created_by,
       t.created_at,
       t.updated_at,
       COALESCE(
               (SELECT JSON_AGG(
                               JSON_BUILD_OBJECT(
                                       'id', tfr.id,
                                       'name', f.name,
                                       'field_type' , f.field_type,
                                       'internal', f.internal,
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
                LEFT JOIN field f ON f.id = tfr.field_id
                WHERE tfr.template_id = t.id), '[]'
       )                                                        as fields
FROM template t
WHERE t.project_id = :project_id
  AND t.internal = :internal;


--! get_fields
SELECT f.id,
       f.name,
       f.field_type,
       f.internal,
       COALESCE(
               (SELECT JSON_AGG(
                               JSON_BUILD_OBJECT(
                                       'id', fo.id,
                                       'value', fo.value,
                                       'position', fo.position
                               )
                       )
                FROM field_option fo
                WHERE fo.field_id = f.id), '[]'
       ) AS options
FROM field f
WHERE f.project_id = :project_id
AND f.internal = :internal;


--! get_field_option_by_id
SELECT id,
       value,
       position
FROM field_option
WHERE field_id = :field_id;
