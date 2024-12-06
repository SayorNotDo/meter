
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
