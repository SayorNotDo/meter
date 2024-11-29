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
                                            'field_id', fo.field_id,
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
                                                            'field_id', fo.field_id,
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
WHERE t.project_id = :project_id;

--! create_field (remark?) :
INSERT INTO field
(name, project_id, field_type, internal, remark, created_by)
VALUES (:name, :project_id, :field_type, :internal, :remark, :created_by)
RETURNING id;

--! update_field (remark?) :
UPDATE field
SET name = :name,
    field_type = :field_type,
    remark = :remark,
    updated_by = :updated_by
WHERE id = :field_id;

--! get_fields : (remark?)
SELECT f.id,
       f.name,
       f.field_type,
       f.project_id,
       f.remark,
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
WHERE f.project_id = :project_id AND deleted_at IS NULL AND deleted_by IS NULL;

--! insert_field_option
INSERT INTO field_option
(field_id, value, position, created_by)
VALUES(:field_id, :value, :position, :created_by)
RETURNING id;

--! get_field_option_by_id
SELECT  id,
        value,
        field_id,
        position
FROM field_option
WHERE id = :option_id;

--! update_field_option
UPDATE field_option
SET value = :value,
    position = :position,
    updated_by = :updated_by
WHERE id = :option_id;

--! soft_delete_field_option
UPDATE field_option
SET deleted_at = NOW(),
    deleted_by = :deleted_by
WHERE id = :id;

--! soft_delete_field_option_by_field_id
UPDATE field_option
SET deleted_at = NOW(),
    deleted_by = :deleted_by
WHERE field_id = :field_id;

--! get_options_by_field_id
SELECT id,
       value,
       field_id,
       position
FROM field_option
WHERE field_id = :field_id AND deleted_at IS NULL AND deleted_by IS NULL;

--! get_field_by_id : (remark?)
SELECT
    f.id,
    f.name,
    f.field_type,
    f.internal,
    f.remark,
    f.project_id,
    COALESCE(
        (SELECT JSON_AGG(
            JSON_BUILD_OBJECT(
                'id', fo.id,
                'value', fo.value,
                'position', fo.position
            )
        ) FROM field_option fo WHERE fo.field_id = f.id), '[]'
    ) AS options
FROM field f
WHERE f.id = :id AND deleted_at IS NULL AND deleted_by IS NULL;

--! soft_delete_field
UPDATE field
SET deleted_at = NOW(),
    deleted_by = :deleted_by
WHERE id = :field_id;
