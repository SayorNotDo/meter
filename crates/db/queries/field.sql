--! get_field_by_id_and_value
SELECT  fcfr.id,
        fcfr.case_id,
        fcfr.field_id,
        f.name
FROM functional_case_field_relation fcfr
LEFT JOIN field f ON f.id = fcfr.field_id
WHERE fcfr.field_id = :field_id
AND fcfr.field_value = :field_value
AND fcfr.deleted_at IS NULL
AND fcfr.deleted_by IS NULL;

--! soft_delete_field_relation_by_case_id
UPDATE functional_case_field_relation
SET deleted_at = NOW(),
    deleted_by = :deleted_by,
    updated_by = :deleted_by
WHERE case_id = :case_id;

--! get_fields_by_case_id : (remark?, options?)
SELECT fcfr.id,
       f.name,
       f.label,
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
