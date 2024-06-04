--! find_project_by_id : (updated_at?, updated_by?, deleted_at?, deleted_by?, description?, module_setting?)
SELECT
    id,
    name,
    organization,
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
WHERE created_by = :uid;

--! insert_project
INSERT INTO
    projects (name, organization, created_by, description, module_setting)
VALUES
    (:name, :organization, :created_by, :description, :module_setting)
RETURNING id;