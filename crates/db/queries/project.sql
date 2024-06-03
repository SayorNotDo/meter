--! find_project_by_id : (updated_at?, updated_by?, deleted_at?, deleted_by?, description?, module_setting?)
SELECT
    id,
    name,
    created_at,
    created_by,
    updated_at,
    updated_by,
    deleted_by,
    deleted_at,
    description,
    module_setting
FROM projects
WHERE id = :id;

--! find_projects_by_uid : (updated_at?, updated_by?, deleted_at?, deleted_by?, description?, module_setting?)
SELECT
    id,
    name,
    created_at,
    created_by,
    updated_at,
    updated_by,
    deleted_by,
    deleted_at,
    description,
    module_setting
FROM projects
WHERE id = :id AND created_by = :uid;