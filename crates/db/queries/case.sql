--! get_case_list
SELECT
    id,
    name,
    module_id,
    tags,
    status,
    created_at,
    created_by,
    updated_at,
    updated_by
FROM functional_cases
WHERE project_id = :project_id AND deleted = FALSE;